/**
 * VALID-001: Differential Testing Framework
 * 
 * Compares outputs from Ruchy self-compilation with reference compiler
 * Uses Deno for all test execution and validation
 */

import { assertEquals, assertExists, assertThrows } from "https://deno.land/std@0.208.0/assert/mod.ts";
import { ensureDir } from "https://deno.land/std@0.208.0/fs/mod.ts";
import { join } from "https://deno.land/std@0.208.0/path/mod.ts";
import { SelfCompilationHarness } from "./self_compilation_harness.ts";

interface DifferentialTestCase {
  name: string;
  sourceFile: string;
  expectedBehavior: "identical" | "equivalent" | "optimized";
  performanceThreshold?: number; // Max acceptable performance ratio
}

interface DifferentialResult {
  testName: string;
  passed: boolean;
  bitIdentical: boolean;
  semanticEquivalent: boolean;
  performanceRatio: number;
  differences: string[];
  executionMatch: boolean;
}

/**
 * Differential Testing Runner
 * Systematically compares self-compiled code with reference compiler
 */
export class DifferentialTestRunner {
  private harness: SelfCompilationHarness;
  private testResults: Map<string, DifferentialResult>;
  private workDir: string;
  
  constructor(workDir = "./build/differential") {
    this.harness = new SelfCompilationHarness(workDir);
    this.testResults = new Map();
    this.workDir = workDir;
  }
  
  /**
   * Initialize the differential testing environment
   */
  async initialize(): Promise<void> {
    await this.harness.initialize();
    await ensureDir(join(this.workDir, "execution"));
    await ensureDir(join(this.workDir, "reports"));
    console.log("✅ Differential testing environment initialized");
  }
  
  /**
   * Run a single differential test case
   */
  async runTest(testCase: DifferentialTestCase): Promise<DifferentialResult> {
    console.log(`\n🔍 Running differential test: ${testCase.name}`);
    
    // Compile with self-hosted compiler
    const selfCompiled = await this.harness.compileSelfHosted(
      testCase.sourceFile,
      `${testCase.name}_self.ts`
    );
    
    // Compile with reference compiler
    const reference = await this.harness.compileReference(
      testCase.sourceFile,
      `${testCase.name}_ref.ts`
    );
    
    // Run differential analysis
    const diffResult = await this.harness.differentialTest(
      selfCompiled,
      reference
    );
    
    // Execute both outputs and compare behavior
    const executionMatch = await this.compareExecution(
      join(this.workDir, "output", `${testCase.name}_self.ts`),
      join(this.workDir, "reference", `${testCase.name}_ref.ts`)
    );
    
    // Build comprehensive result
    const result: DifferentialResult = {
      testName: testCase.name,
      passed: this.evaluateTestCase(testCase, diffResult, executionMatch),
      bitIdentical: diffResult.metrics.bitIdentical,
      semanticEquivalent: diffResult.metrics.semanticEquivalent,
      performanceRatio: diffResult.metrics.performanceRatio,
      differences: diffResult.differences,
      executionMatch
    };
    
    this.testResults.set(testCase.name, result);
    
    // Log result summary
    console.log(`  Status: ${result.passed ? "✅ PASS" : "❌ FAIL"}`);
    console.log(`  Bit Identical: ${result.bitIdentical}`);
    console.log(`  Semantic Equivalent: ${result.semanticEquivalent}`);
    console.log(`  Execution Match: ${result.executionMatch}`);
    console.log(`  Performance: ${result.performanceRatio.toFixed(2)}x`);
    
    return result;
  }
  
  /**
   * Evaluate if test case passes based on expected behavior
   */
  private evaluateTestCase(
    testCase: DifferentialTestCase,
    diffResult: { pass: boolean; metrics: any },
    executionMatch: boolean
  ): boolean {
    switch (testCase.expectedBehavior) {
      case "identical":
        // Must be bit-identical
        return diffResult.metrics.bitIdentical;
        
      case "equivalent":
        // Must be semantically equivalent and execute the same
        return diffResult.metrics.semanticEquivalent && executionMatch;
        
      case "optimized":
        // Can differ but must execute correctly and meet performance
        const performanceOk = testCase.performanceThreshold
          ? diffResult.metrics.performanceRatio <= testCase.performanceThreshold
          : true;
        return executionMatch && performanceOk;
        
      default:
        return diffResult.pass;
    }
  }
  
  /**
   * Compare execution behavior of two TypeScript files
   */
  private async compareExecution(
    selfCompiledPath: string,
    referencePath: string
  ): Promise<boolean> {
    try {
      // Execute both with Deno
      const selfResult = await this.executeTypeScript(selfCompiledPath);
      const refResult = await this.executeTypeScript(referencePath);
      
      // Compare outputs
      return selfResult.stdout === refResult.stdout &&
             selfResult.stderr === refResult.stderr &&
             selfResult.exitCode === refResult.exitCode;
    } catch (error) {
      console.error(`Execution comparison failed: ${error.message}`);
      return false;
    }
  }
  
  /**
   * Execute TypeScript file with Deno and capture output
   */
  private async executeTypeScript(
    filePath: string
  ): Promise<{ stdout: string; stderr: string; exitCode: number }> {
    const process = new Deno.Command("deno", {
      args: ["run", "--allow-all", filePath],
      stdout: "piped",
      stderr: "piped"
    });
    
    const { code, stdout, stderr } = await process.output();
    
    return {
      stdout: new TextDecoder().decode(stdout),
      stderr: new TextDecoder().decode(stderr),
      exitCode: code
    };
  }
  
  /**
   * Run a suite of differential tests
   */
  async runTestSuite(testCases: DifferentialTestCase[]): Promise<void> {
    console.log(`\n📋 Running differential test suite with ${testCases.length} tests`);
    
    for (const testCase of testCases) {
      await this.runTest(testCase);
    }
    
    // Generate summary report
    const report = this.generateReport();
    await this.saveReport(report);
    
    // Print summary
    this.printSummary();
  }
  
  /**
   * Generate comprehensive differential testing report
   */
  private generateReport(): string {
    let report = "# Differential Testing Report\n\n";
    report += `Generated: ${new Date().toISOString()}\n\n`;
    
    report += "## Test Results\n\n";
    
    let totalTests = 0;
    let passedTests = 0;
    let identicalCount = 0;
    let equivalentCount = 0;
    let executionMatchCount = 0;
    
    for (const [name, result] of this.testResults) {
      totalTests++;
      if (result.passed) passedTests++;
      if (result.bitIdentical) identicalCount++;
      if (result.semanticEquivalent) equivalentCount++;
      if (result.executionMatch) executionMatchCount++;
      
      report += `### ${name}\n`;
      report += `- **Status**: ${result.passed ? "✅ PASS" : "❌ FAIL"}\n`;
      report += `- **Bit Identical**: ${result.bitIdentical}\n`;
      report += `- **Semantic Equivalent**: ${result.semanticEquivalent}\n`;
      report += `- **Execution Match**: ${result.executionMatch}\n`;
      report += `- **Performance Ratio**: ${result.performanceRatio.toFixed(2)}x\n`;
      
      if (result.differences.length > 0) {
        report += `- **Differences Found**:\n`;
        for (const diff of result.differences.slice(0, 3)) {
          report += `  - ${diff}\n`;
        }
      }
      report += "\n";
    }
    
    report += "## Summary Statistics\n\n";
    report += `- **Total Tests**: ${totalTests}\n`;
    report += `- **Passed**: ${passedTests} (${((passedTests/totalTests)*100).toFixed(1)}%)\n`;
    report += `- **Bit Identical**: ${identicalCount} (${((identicalCount/totalTests)*100).toFixed(1)}%)\n`;
    report += `- **Semantic Equivalent**: ${equivalentCount} (${((equivalentCount/totalTests)*100).toFixed(1)}%)\n`;
    report += `- **Execution Match**: ${executionMatchCount} (${((executionMatchCount/totalTests)*100).toFixed(1)}%)\n`;
    
    return report;
  }
  
  /**
   * Save report to file
   */
  private async saveReport(report: string): Promise<void> {
    const reportPath = join(this.workDir, "reports", "differential_report.md");
    await Deno.writeTextFile(reportPath, report);
    console.log(`\n📊 Report saved to: ${reportPath}`);
  }
  
  /**
   * Print test summary to console
   */
  private printSummary(): void {
    const results = Array.from(this.testResults.values());
    const passed = results.filter(r => r.passed).length;
    const total = results.length;
    
    console.log("\n" + "=".repeat(50));
    console.log("DIFFERENTIAL TESTING SUMMARY");
    console.log("=".repeat(50));
    console.log(`Total Tests: ${total}`);
    console.log(`Passed: ${passed}/${total} (${((passed/total)*100).toFixed(1)}%)`);
    
    if (passed === total) {
      console.log("\n✅ All differential tests passed!");
    } else {
      console.log("\n❌ Some differential tests failed");
      console.log("Failed tests:");
      results
        .filter(r => !r.passed)
        .forEach(r => console.log(`  - ${r.testName}`));
    }
  }
}

/**
 * Standard differential test suite
 */
export async function runStandardDifferentialTests(): Promise<void> {
  const runner = new DifferentialTestRunner();
  await runner.initialize();
  
  const testCases: DifferentialTestCase[] = [
    {
      name: "lexer_self_compilation",
      sourceFile: "bootstrap/stage0/lexer.ruchy",
      expectedBehavior: "identical"
    },
    {
      name: "parser_self_compilation",
      sourceFile: "bootstrap/stage1/parser.ruchy",
      expectedBehavior: "equivalent",
      performanceThreshold: 1.5
    },
    {
      name: "type_checker_self_compilation",
      sourceFile: "bootstrap/stage2/type_checker.ruchy",
      expectedBehavior: "equivalent",
      performanceThreshold: 2.0
    },
    {
      name: "code_generator_self_compilation",
      sourceFile: "bootstrap/stage3/code_generator.ruchy",
      expectedBehavior: "optimized",
      performanceThreshold: 2.5
    }
  ];
  
  await runner.runTestSuite(testCases);
}

// Run if main module
if (import.meta.main) {
  await runStandardDifferentialTests();
}