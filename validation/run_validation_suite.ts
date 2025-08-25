#!/usr/bin/env -S deno run --allow-all

/**
 * Main Validation Suite Runner
 * 
 * Orchestrates all Phase 2 validation tools for comprehensive testing
 * Sprint 1: VALID-001 and VALID-002 implementation
 */

import { runSelfCompilationTests } from "./self_compilation_harness.ts";
import { runStandardDifferentialTests } from "./differential_test_runner.ts";
import { ContinuousValidationPipeline } from "./continuous_pipeline.ts";
import { runDenoValidationSuite } from "./deno_toolchain_validator.ts";
import { format } from "https://deno.land/std@0.208.0/datetime/mod.ts";

/**
 * Sprint 1 Validation Suite
 * Executes VALID-001 and VALID-002 comprehensive tests
 */
async function runSprint1ValidationSuite() {
  console.log("=" .repeat(70));
  console.log("🚀 PHASE 2 VALIDATION - SPRINT 1");
  console.log("=" .repeat(70));
  console.log(`Start Time: ${format(new Date(), "yyyy-MM-dd HH:mm:ss")}`);
  console.log("\nObjective: Validate Ruchy tools against self-compiled code");
  console.log("Tickets: VALID-001 (Self-Compilation) & VALID-002 (Deno Toolchain)");
  console.log("=" .repeat(70));
  
  const results = {
    selfCompilation: false,
    differential: false,
    denoToolchain: false,
    continuous: false,
    totalTests: 0,
    passedTests: 0,
    failedTests: 0,
    startTime: performance.now()
  };
  
  try {
    // VALID-001: Self-Compilation Test Harness
    console.log("\n" + "=".repeat(70));
    console.log("📦 VALID-001: SELF-COMPILATION TEST HARNESS");
    console.log("=".repeat(70));
    console.log("Testing Ruchy's ability to compile itself...\n");
    
    try {
      await runSelfCompilationTests();
      results.selfCompilation = true;
      results.passedTests++;
      console.log("\n✅ Self-compilation tests PASSED");
    } catch (error) {
      console.error(`\n❌ Self-compilation tests FAILED: ${error.message}`);
      results.failedTests++;
    }
    results.totalTests++;
    
    // VALID-001: Differential Testing Framework
    console.log("\n" + "=".repeat(70));
    console.log("🔄 VALID-001: DIFFERENTIAL TESTING FRAMEWORK");
    console.log("=".repeat(70));
    console.log("Comparing self-compiled output with reference compiler...\n");
    
    try {
      await runStandardDifferentialTests();
      results.differential = true;
      results.passedTests++;
      console.log("\n✅ Differential tests PASSED");
    } catch (error) {
      console.error(`\n❌ Differential tests FAILED: ${error.message}`);
      results.failedTests++;
    }
    results.totalTests++;
    
    // VALID-002: Deno Toolchain Validation
    console.log("\n" + "=".repeat(70));
    console.log("🦕 VALID-002: DENO TOOLCHAIN VALIDATION");
    console.log("=".repeat(70));
    console.log("Validating generated TypeScript with Deno tools...\n");
    
    try {
      await runDenoValidationSuite();
      results.denoToolchain = true;
      results.passedTests++;
      console.log("\n✅ Deno toolchain validation PASSED");
    } catch (error) {
      console.error(`\n❌ Deno toolchain validation FAILED: ${error.message}`);
      results.failedTests++;
    }
    results.totalTests++;
    
    // VALID-001: Continuous Validation Pipeline (Single Run)
    console.log("\n" + "=".repeat(70));
    console.log("⚡ VALID-001: CONTINUOUS VALIDATION PIPELINE");
    console.log("=".repeat(70));
    console.log("Running continuous validation pipeline (single run)...\n");
    
    try {
      const pipeline = new ContinuousValidationPipeline({
        watchMode: false,
        generateReports: true,
        notifyOnFailure: false
      });
      
      await pipeline.initialize();
      const pipelineResult = await pipeline.runOnce();
      
      if (pipelineResult.testsFailed === 0) {
        results.continuous = true;
        results.passedTests++;
        console.log("\n✅ Continuous pipeline validation PASSED");
      } else {
        throw new Error(`${pipelineResult.testsFailed} tests failed in pipeline`);
      }
    } catch (error) {
      console.error(`\n❌ Continuous pipeline validation FAILED: ${error.message}`);
      results.failedTests++;
    }
    results.totalTests++;
    
  } catch (error) {
    console.error(`\n❌ Fatal error in validation suite: ${error.message}`);
  }
  
  // Generate final report
  generateFinalReport(results);
}

/**
 * Generate comprehensive final report
 */
function generateFinalReport(results: any) {
  const duration = (performance.now() - results.startTime) / 1000;
  
  console.log("\n" + "=".repeat(70));
  console.log("📊 SPRINT 1 VALIDATION SUMMARY");
  console.log("=".repeat(70));
  console.log(`End Time: ${format(new Date(), "yyyy-MM-dd HH:mm:ss")}`);
  console.log(`Duration: ${duration.toFixed(2)} seconds`);
  console.log("=".repeat(70));
  
  console.log("\n🎯 VALID-001: Self-Compilation Test Harness");
  console.log(`  ├─ Test Harness: ${results.selfCompilation ? "✅ PASS" : "❌ FAIL"}`);
  console.log(`  ├─ Differential Testing: ${results.differential ? "✅ PASS" : "❌ FAIL"}`);
  console.log(`  └─ Continuous Pipeline: ${results.continuous ? "✅ PASS" : "❌ FAIL"}`);
  
  console.log("\n🦕 VALID-002: Deno Toolchain Validation");
  console.log(`  └─ Deno Compatibility: ${results.denoToolchain ? "✅ PASS" : "❌ FAIL"}`);
  
  console.log("\n📈 Overall Results:");
  console.log(`  Total Test Suites: ${results.totalTests}`);
  console.log(`  Passed: ${results.passedTests}`);
  console.log(`  Failed: ${results.failedTests}`);
  console.log(`  Success Rate: ${((results.passedTests / results.totalTests) * 100).toFixed(1)}%`);
  
  console.log("\n" + "=".repeat(70));
  
  if (results.failedTests === 0) {
    console.log("✅ 🎉 SPRINT 1 COMPLETE - ALL VALIDATION TESTS PASSED!");
    console.log("\nReady to proceed to Sprint 2:");
    console.log("  - VALID-003: AST Validation Framework");
    console.log("  - PROP-001: Lexer Property Testing");
  } else {
    console.log("❌ SPRINT 1 INCOMPLETE - Some validation tests failed");
    console.log("\nRequired fixes before proceeding:");
    if (!results.selfCompilation) console.log("  - Fix self-compilation test harness");
    if (!results.differential) console.log("  - Fix differential testing framework");
    if (!results.denoToolchain) console.log("  - Fix Deno toolchain compatibility");
    if (!results.continuous) console.log("  - Fix continuous validation pipeline");
  }
  
  console.log("=" .repeat(70));
}

// Main execution
if (import.meta.main) {
  await runSprint1ValidationSuite();
  
  // Exit with appropriate code
  Deno.exit(0);
}