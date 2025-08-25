#!/usr/bin/env -S deno run --allow-all

/**
 * VALID-002: Deno Toolchain Validation
 * 
 * Validates Ruchy-generated TypeScript/JavaScript with Deno toolchain
 * Tests: deno run, deno fmt, deno lint, deno test, deno bench
 */

import { assertEquals } from "https://deno.land/std@0.208.0/assert/mod.ts";
import { ensureDir } from "https://deno.land/std@0.208.0/fs/mod.ts";
import { join } from "https://deno.land/std@0.208.0/path/mod.ts";
import { format } from "https://deno.land/std@0.208.0/datetime/mod.ts";

interface DenoValidationResult {
  tool: "run" | "fmt" | "lint" | "test" | "bench" | "check";
  success: boolean;
  output: string;
  errors: string[];
  warnings: string[];
  metrics?: {
    executionTime?: number;
    memoryUsage?: number;
    formattingChanges?: number;
    lintIssues?: number;
    testsRun?: number;
    testsPassed?: number;
  };
}

interface ValidationSummary {
  timestamp: Date;
  totalTests: number;
  passed: number;
  failed: number;
  toolResults: Map<string, DenoValidationResult>;
  compatibility: {
    denoRun: boolean;
    denoFmt: boolean;
    denoLint: boolean;
    denoTest: boolean;
    denoBench: boolean;
    denoCheck: boolean;
  };
}

/**
 * Deno Toolchain Validator
 * Comprehensive validation of Ruchy-generated code with Deno tools
 */
export class DenoToolchainValidator {
  private workDir: string;
  private results: Map<string, DenoValidationResult>;
  
  constructor(workDir = "./build/deno_validation") {
    this.workDir = workDir;
    this.results = new Map();
  }
  
  /**
   * Initialize validation environment
   */
  async initialize(): Promise<void> {
    await ensureDir(this.workDir);
    await ensureDir(join(this.workDir, "source"));
    await ensureDir(join(this.workDir, "formatted"));
    await ensureDir(join(this.workDir, "tests"));
    await ensureDir(join(this.workDir, "benchmarks"));
    await ensureDir(join(this.workDir, "reports"));
    
    // Verify Deno is available
    const denoVersion = await this.getDenoVersion();
    console.log(`✅ Deno ${denoVersion} detected`);
    console.log(`📁 Working directory: ${this.workDir}`);
  }
  
  /**
   * Get Deno version
   */
  private async getDenoVersion(): Promise<string> {
    const process = new Deno.Command("deno", {
      args: ["--version"],
      stdout: "piped"
    });
    
    const { stdout } = await process.output();
    const output = new TextDecoder().decode(stdout);
    const match = output.match(/deno (\d+\.\d+\.\d+)/);
    return match ? match[1] : "unknown";
  }
  
  /**
   * Validate TypeScript file with deno run
   */
  async validateDenoRun(filePath: string): Promise<DenoValidationResult> {
    console.log(`\n🏃 Testing with deno run: ${filePath}`);
    
    const startTime = performance.now();
    const result: DenoValidationResult = {
      tool: "run",
      success: false,
      output: "",
      errors: [],
      warnings: [],
      metrics: {}
    };
    
    try {
      const process = new Deno.Command("deno", {
        args: ["run", "--allow-all", "--check", filePath],
        stdout: "piped",
        stderr: "piped"
      });
      
      const { code, stdout, stderr } = await process.output();
      
      result.output = new TextDecoder().decode(stdout);
      const errorOutput = new TextDecoder().decode(stderr);
      
      result.success = code === 0;
      result.metrics!.executionTime = performance.now() - startTime;
      
      if (!result.success) {
        result.errors = errorOutput.split("\n").filter(line => line.trim());
      }
      
      // Check for warnings in stderr
      const warnings = errorOutput
        .split("\n")
        .filter(line => line.includes("warning") || line.includes("Warning"));
      result.warnings = warnings;
      
      // Estimate memory usage
      const memInfo = Deno.memoryUsage();
      result.metrics!.memoryUsage = memInfo.rss / (1024 * 1024); // MB
      
      console.log(`  Result: ${result.success ? "✅ SUCCESS" : "❌ FAILED"}`);
      console.log(`  Execution time: ${result.metrics!.executionTime!.toFixed(2)}ms`);
      
      if (result.errors.length > 0) {
        console.log(`  Errors: ${result.errors.length}`);
        result.errors.slice(0, 3).forEach(err => 
          console.log(`    - ${err.substring(0, 80)}`));
      }
      
    } catch (error) {
      result.errors = [error.message];
      console.log(`  ❌ Exception: ${error.message}`);
    }
    
    this.results.set(`run_${filePath}`, result);
    return result;
  }
  
  /**
   * Validate formatting with deno fmt
   */
  async validateDenoFmt(filePath: string): Promise<DenoValidationResult> {
    console.log(`\n🎨 Testing with deno fmt: ${filePath}`);
    
    const result: DenoValidationResult = {
      tool: "fmt",
      success: false,
      output: "",
      errors: [],
      warnings: [],
      metrics: {}
    };
    
    try {
      // First, check formatting without modifying
      const checkProcess = new Deno.Command("deno", {
        args: ["fmt", "--check", filePath],
        stdout: "piped",
        stderr: "piped"
      });
      
      const checkResult = await checkProcess.output();
      const checkOutput = new TextDecoder().decode(checkResult.stdout);
      
      // Count formatting issues
      const formattingIssues = checkOutput
        .split("\n")
        .filter(line => line.includes("would be reformatted"));
      result.metrics!.formattingChanges = formattingIssues.length;
      
      // Now actually format and save to formatted directory
      const formattedPath = join(this.workDir, "formatted", `formatted_${filePath.split("/").pop()}`);
      const content = await Deno.readTextFile(filePath);
      await Deno.writeTextFile(formattedPath, content);
      
      const formatProcess = new Deno.Command("deno", {
        args: ["fmt", formattedPath],
        stdout: "piped",
        stderr: "piped"
      });
      
      const { code, stderr } = await formatProcess.output();
      
      result.success = code === 0;
      
      if (!result.success) {
        const errorOutput = new TextDecoder().decode(stderr);
        result.errors = errorOutput.split("\n").filter(line => line.trim());
      }
      
      // Compare original and formatted
      const formattedContent = await Deno.readTextFile(formattedPath);
      const identical = content === formattedContent;
      
      console.log(`  Result: ${result.success ? "✅ SUCCESS" : "❌ FAILED"}`);
      console.log(`  Format changes needed: ${!identical}`);
      console.log(`  Issues found: ${result.metrics!.formattingChanges}`);
      
      if (!identical) {
        result.warnings.push("Code requires formatting changes");
        
        // Show first few differences
        const origLines = content.split("\n");
        const fmtLines = formattedContent.split("\n");
        let diffCount = 0;
        
        for (let i = 0; i < Math.min(origLines.length, fmtLines.length); i++) {
          if (origLines[i] !== fmtLines[i] && diffCount < 3) {
            console.log(`    Line ${i + 1} differs`);
            diffCount++;
          }
        }
      }
      
    } catch (error) {
      result.errors = [error.message];
      console.log(`  ❌ Exception: ${error.message}`);
    }
    
    this.results.set(`fmt_${filePath}`, result);
    return result;
  }
  
  /**
   * Validate with deno lint
   */
  async validateDenoLint(filePath: string): Promise<DenoValidationResult> {
    console.log(`\n🔍 Testing with deno lint: ${filePath}`);
    
    const result: DenoValidationResult = {
      tool: "lint",
      success: false,
      output: "",
      errors: [],
      warnings: [],
      metrics: {}
    };
    
    try {
      const process = new Deno.Command("deno", {
        args: ["lint", filePath],
        stdout: "piped",
        stderr: "piped"
      });
      
      const { code, stdout, stderr } = await process.output();
      
      result.output = new TextDecoder().decode(stdout);
      const errorOutput = new TextDecoder().decode(stderr);
      
      result.success = code === 0;
      
      // Parse lint issues
      const lintIssues = result.output
        .split("\n")
        .filter(line => line.includes("(") && line.includes(")"));
      
      result.metrics!.lintIssues = lintIssues.length;
      
      if (lintIssues.length > 0) {
        result.warnings = lintIssues.slice(0, 10); // First 10 issues
      }
      
      if (!result.success && errorOutput) {
        result.errors = errorOutput.split("\n").filter(line => line.trim());
      }
      
      console.log(`  Result: ${result.success ? "✅ SUCCESS" : "❌ FAILED"}`);
      console.log(`  Lint issues: ${result.metrics!.lintIssues}`);
      
      if (result.warnings.length > 0) {
        console.log(`  Sample issues:`);
        result.warnings.slice(0, 3).forEach(warning => 
          console.log(`    - ${warning.substring(0, 80)}`));
      }
      
    } catch (error) {
      result.errors = [error.message];
      console.log(`  ❌ Exception: ${error.message}`);
    }
    
    this.results.set(`lint_${filePath}`, result);
    return result;
  }
  
  /**
   * Validate with deno test
   */
  async validateDenoTest(testFilePath: string): Promise<DenoValidationResult> {
    console.log(`\n🧪 Testing with deno test: ${testFilePath}`);
    
    const result: DenoValidationResult = {
      tool: "test",
      success: false,
      output: "",
      errors: [],
      warnings: [],
      metrics: {}
    };
    
    try {
      const process = new Deno.Command("deno", {
        args: ["test", "--allow-all", testFilePath],
        stdout: "piped",
        stderr: "piped"
      });
      
      const { code, stdout, stderr } = await process.output();
      
      result.output = new TextDecoder().decode(stdout);
      const errorOutput = new TextDecoder().decode(stderr);
      
      result.success = code === 0;
      
      // Parse test results
      const testRunMatch = result.output.match(/(\d+) passed.*(\d+) failed/);
      if (testRunMatch) {
        result.metrics!.testsPassed = parseInt(testRunMatch[1]);
        result.metrics!.testsRun = result.metrics!.testsPassed + parseInt(testRunMatch[2]);
      }
      
      if (!result.success && errorOutput) {
        result.errors = errorOutput.split("\n").filter(line => line.trim());
      }
      
      console.log(`  Result: ${result.success ? "✅ SUCCESS" : "❌ FAILED"}`);
      if (result.metrics!.testsRun) {
        console.log(`  Tests: ${result.metrics!.testsPassed}/${result.metrics!.testsRun} passed`);
      }
      
    } catch (error) {
      result.errors = [error.message];
      console.log(`  ❌ Exception: ${error.message}`);
    }
    
    this.results.set(`test_${testFilePath}`, result);
    return result;
  }
  
  /**
   * Validate with deno bench
   */
  async validateDenoBench(benchFilePath: string): Promise<DenoValidationResult> {
    console.log(`\n⚡ Testing with deno bench: ${benchFilePath}`);
    
    const result: DenoValidationResult = {
      tool: "bench",
      success: false,
      output: "",
      errors: [],
      warnings: [],
      metrics: {}
    };
    
    try {
      const process = new Deno.Command("deno", {
        args: ["bench", "--allow-all", benchFilePath],
        stdout: "piped",
        stderr: "piped"
      });
      
      const { code, stdout, stderr } = await process.output();
      
      result.output = new TextDecoder().decode(stdout);
      const errorOutput = new TextDecoder().decode(stderr);
      
      result.success = code === 0;
      
      if (!result.success && errorOutput) {
        result.errors = errorOutput.split("\n").filter(line => line.trim());
      }
      
      console.log(`  Result: ${result.success ? "✅ SUCCESS" : "❌ FAILED"}`);
      
    } catch (error) {
      result.errors = [error.message];
      console.log(`  ❌ Exception: ${error.message}`);
    }
    
    this.results.set(`bench_${benchFilePath}`, result);
    return result;
  }
  
  /**
   * Validate with deno check (type checking)
   */
  async validateDenoCheck(filePath: string): Promise<DenoValidationResult> {
    console.log(`\n✔️ Testing with deno check: ${filePath}`);
    
    const result: DenoValidationResult = {
      tool: "check",
      success: false,
      output: "",
      errors: [],
      warnings: [],
      metrics: {}
    };
    
    try {
      const process = new Deno.Command("deno", {
        args: ["check", filePath],
        stdout: "piped",
        stderr: "piped"
      });
      
      const { code, stdout, stderr } = await process.output();
      
      result.output = new TextDecoder().decode(stdout);
      const errorOutput = new TextDecoder().decode(stderr);
      
      result.success = code === 0;
      
      if (!result.success) {
        // Parse TypeScript errors
        const tsErrors = errorOutput
          .split("\n")
          .filter(line => line.includes("TS") || line.includes("error"));
        result.errors = tsErrors;
      }
      
      console.log(`  Result: ${result.success ? "✅ SUCCESS" : "❌ FAILED"}`);
      
      if (result.errors.length > 0) {
        console.log(`  Type errors: ${result.errors.length}`);
        result.errors.slice(0, 3).forEach(err => 
          console.log(`    - ${err.substring(0, 80)}`));
      }
      
    } catch (error) {
      result.errors = [error.message];
      console.log(`  ❌ Exception: ${error.message}`);
    }
    
    this.results.set(`check_${filePath}`, result);
    return result;
  }
  
  /**
   * Run full Deno toolchain validation
   */
  async validateFullToolchain(sourceFile: string): Promise<ValidationSummary> {
    console.log("\n" + "=".repeat(60));
    console.log(`DENO TOOLCHAIN VALIDATION: ${sourceFile}`);
    console.log("=".repeat(60));
    
    const summary: ValidationSummary = {
      timestamp: new Date(),
      totalTests: 0,
      passed: 0,
      failed: 0,
      toolResults: new Map(),
      compatibility: {
        denoRun: false,
        denoFmt: false,
        denoLint: false,
        denoTest: false,
        denoBench: false,
        denoCheck: false
      }
    };
    
    // Copy source file to work directory
    const fileName = sourceFile.split("/").pop()!;
    const workFile = join(this.workDir, "source", fileName);
    const sourceContent = await Deno.readTextFile(sourceFile);
    await Deno.writeTextFile(workFile, sourceContent);
    
    // 1. Type checking with deno check
    const checkResult = await this.validateDenoCheck(workFile);
    summary.toolResults.set("check", checkResult);
    summary.compatibility.denoCheck = checkResult.success;
    summary.totalTests++;
    if (checkResult.success) summary.passed++;
    else summary.failed++;
    
    // 2. Run validation
    const runResult = await this.validateDenoRun(workFile);
    summary.toolResults.set("run", runResult);
    summary.compatibility.denoRun = runResult.success;
    summary.totalTests++;
    if (runResult.success) summary.passed++;
    else summary.failed++;
    
    // 3. Format validation
    const fmtResult = await this.validateDenoFmt(workFile);
    summary.toolResults.set("fmt", fmtResult);
    summary.compatibility.denoFmt = fmtResult.success;
    summary.totalTests++;
    if (fmtResult.success) summary.passed++;
    else summary.failed++;
    
    // 4. Lint validation
    const lintResult = await this.validateDenoLint(workFile);
    summary.toolResults.set("lint", lintResult);
    summary.compatibility.denoLint = lintResult.success;
    summary.totalTests++;
    if (lintResult.success) summary.passed++;
    else summary.failed++;
    
    // 5. Create and run a test file if applicable
    if (sourceContent.includes("export")) {
      const testFile = await this.createTestFile(workFile, fileName);
      const testResult = await this.validateDenoTest(testFile);
      summary.toolResults.set("test", testResult);
      summary.compatibility.denoTest = testResult.success;
      summary.totalTests++;
      if (testResult.success) summary.passed++;
      else summary.failed++;
    }
    
    // Generate report
    await this.generateReport(summary, sourceFile);
    
    // Print summary
    this.printSummary(summary);
    
    return summary;
  }
  
  /**
   * Create a test file for the source
   */
  private async createTestFile(sourceFile: string, fileName: string): Promise<string> {
    const testContent = `
import { assertEquals } from "https://deno.land/std@0.208.0/assert/mod.ts";

// Basic test for ${fileName}
Deno.test("${fileName} loads without errors", async () => {
  const module = await import("./${fileName}");
  assertEquals(typeof module, "object");
});

Deno.test("${fileName} exports are defined", async () => {
  const module = await import("./${fileName}");
  const exports = Object.keys(module);
  assertEquals(exports.length > 0, true);
});
`;
    
    const testPath = join(this.workDir, "tests", `${fileName}.test.ts`);
    await Deno.writeTextFile(testPath, testContent);
    return testPath;
  }
  
  /**
   * Generate validation report
   */
  private async generateReport(summary: ValidationSummary, sourceFile: string): Promise<void> {
    const timestamp = format(summary.timestamp, "yyyyMMdd_HHmmss");
    const reportPath = join(this.workDir, "reports", `deno_validation_${timestamp}.md`);
    
    let report = "# Deno Toolchain Validation Report\n\n";
    report += `**Source File**: ${sourceFile}\n`;
    report += `**Timestamp**: ${format(summary.timestamp, "yyyy-MM-dd HH:mm:ss")}\n\n`;
    
    report += "## Summary\n\n";
    report += `- **Total Tests**: ${summary.totalTests}\n`;
    report += `- **Passed**: ${summary.passed} ✅\n`;
    report += `- **Failed**: ${summary.failed} ❌\n`;
    report += `- **Success Rate**: ${((summary.passed / summary.totalTests) * 100).toFixed(1)}%\n\n`;
    
    report += "## Compatibility Matrix\n\n";
    report += "| Tool | Status | Compatible |\n";
    report += "|------|--------|------------|\n`;
    report += `| deno check | ${summary.compatibility.denoCheck ? "✅" : "❌"} | ${summary.compatibility.denoCheck ? "Yes" : "No"} |\n`;
    report += `| deno run | ${summary.compatibility.denoRun ? "✅" : "❌"} | ${summary.compatibility.denoRun ? "Yes" : "No"} |\n`;
    report += `| deno fmt | ${summary.compatibility.denoFmt ? "✅" : "❌"} | ${summary.compatibility.denoFmt ? "Yes" : "No"} |\n`;
    report += `| deno lint | ${summary.compatibility.denoLint ? "✅" : "❌"} | ${summary.compatibility.denoLint ? "Yes" : "No"} |\n`;
    report += `| deno test | ${summary.compatibility.denoTest ? "✅" : "❌"} | ${summary.compatibility.denoTest ? "Yes" : "No"} |\n\n`;
    
    report += "## Detailed Results\n\n";
    
    for (const [tool, result] of summary.toolResults) {
      report += `### ${tool.toUpperCase()}\n`;
      report += `- **Success**: ${result.success ? "Yes" : "No"}\n`;
      
      if (result.metrics) {
        if (result.metrics.executionTime !== undefined) {
          report += `- **Execution Time**: ${result.metrics.executionTime.toFixed(2)}ms\n`;
        }
        if (result.metrics.lintIssues !== undefined) {
          report += `- **Lint Issues**: ${result.metrics.lintIssues}\n`;
        }
        if (result.metrics.formattingChanges !== undefined) {
          report += `- **Formatting Changes**: ${result.metrics.formattingChanges}\n`;
        }
        if (result.metrics.testsRun !== undefined) {
          report += `- **Tests Run**: ${result.metrics.testsRun}\n`;
          report += `- **Tests Passed**: ${result.metrics.testsPassed}\n`;
        }
      }
      
      if (result.errors.length > 0) {
        report += `- **Errors**:\n`;
        result.errors.slice(0, 5).forEach(err => 
          report += `  - ${err}\n`);
      }
      
      if (result.warnings.length > 0) {
        report += `- **Warnings**:\n`;
        result.warnings.slice(0, 5).forEach(warn => 
          report += `  - ${warn}\n`);
      }
      
      report += "\n";
    }
    
    await Deno.writeTextFile(reportPath, report);
    console.log(`\n📊 Report saved to: ${reportPath}`);
  }
  
  /**
   * Print validation summary
   */
  private printSummary(summary: ValidationSummary): void {
    console.log("\n" + "=".repeat(60));
    console.log("DENO TOOLCHAIN VALIDATION SUMMARY");
    console.log("=".repeat(60));
    
    console.log(`Total: ${summary.passed}/${summary.totalTests} tools compatible`);
    
    console.log("\nCompatibility:");
    console.log(`  deno check: ${summary.compatibility.denoCheck ? "✅" : "❌"}`);
    console.log(`  deno run:   ${summary.compatibility.denoRun ? "✅" : "❌"}`);
    console.log(`  deno fmt:   ${summary.compatibility.denoFmt ? "✅" : "❌"}`);
    console.log(`  deno lint:  ${summary.compatibility.denoLint ? "✅" : "❌"}`);
    console.log(`  deno test:  ${summary.compatibility.denoTest ? "✅" : "❌"}`);
    
    if (summary.passed === summary.totalTests) {
      console.log("\n✅ Full Deno toolchain compatibility achieved!");
    } else {
      console.log(`\n⚠️ ${summary.failed} Deno tools have compatibility issues`);
    }
  }
}

/**
 * Run Deno toolchain validation suite
 */
export async function runDenoValidationSuite(): Promise<void> {
  const validator = new DenoToolchainValidator();
  await validator.initialize();
  
  // Test files to validate
  const testFiles = [
    "validation/self_compilation_harness.ts",
    "validation/differential_test_runner.ts",
    "validation/output_comparator.ts",
    "validation/continuous_pipeline.ts"
  ];
  
  for (const file of testFiles) {
    await validator.validateFullToolchain(file);
  }
}

// CLI interface
if (import.meta.main) {
  if (Deno.args.length === 0) {
    await runDenoValidationSuite();
  } else {
    const validator = new DenoToolchainValidator();
    await validator.initialize();
    
    for (const file of Deno.args) {
      await validator.validateFullToolchain(file);
    }
  }
}