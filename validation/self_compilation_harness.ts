/**
 * VALID-001: Self-Compilation Test Harness
 * 
 * Tests Ruchy tools against code compiled by Ruchy itself.
 * Uses Deno for all test execution and validation.
 */

import { assertEquals, assertExists } from "https://deno.land/std@0.208.0/assert/mod.ts";
import { ensureDir } from "https://deno.land/std@0.208.0/fs/mod.ts";
import { join } from "https://deno.land/std@0.208.0/path/mod.ts";

interface CompilationResult {
  success: boolean;
  output: string;
  errors: string[];
  stats: {
    compilationTime: number;
    outputSize: number;
    linesOfCode: number;
  };
}

interface ValidationResult {
  pass: boolean;
  differences: string[];
  metrics: {
    bitIdentical: boolean;
    semanticEquivalent: boolean;
    performanceRatio: number;
  };
}

/**
 * Self-Compilation Test Harness
 * Compiles Ruchy code using Ruchy itself and validates the output
 */
export class SelfCompilationHarness {
  private readonly workDir: string;
  private readonly ruchyBinary: string;
  
  constructor(workDir = "./build/validation", ruchyBinary = "ruchy") {
    this.workDir = workDir;
    this.ruchyBinary = ruchyBinary;
  }
  
  /**
   * Initialize the test harness environment
   */
  async initialize(): Promise<void> {
    await ensureDir(this.workDir);
    await ensureDir(join(this.workDir, "input"));
    await ensureDir(join(this.workDir, "output"));
    await ensureDir(join(this.workDir, "reference"));
    
    // Verify Ruchy compiler is available
    const ruchyCheck = await this.runCommand(this.ruchyBinary, ["--version"]);
    if (!ruchyCheck.success) {
      throw new Error(`Ruchy compiler not found at ${this.ruchyBinary}`);
    }
    console.log(`✅ Ruchy compiler found: ${ruchyCheck.output}`);
  }
  
  /**
   * Compile Ruchy source code using Ruchy itself
   */
  async compileSelfHosted(
    sourceFile: string,
    outputFile: string
  ): Promise<CompilationResult> {
    const startTime = performance.now();
    
    // First pass: Compile with Ruchy to intermediate representation
    const firstPass = await this.runCommand(this.ruchyBinary, [
      "compile",
      "--target", "typescript",
      "--output", join(this.workDir, "output", outputFile + ".intermediate.ts"),
      sourceFile
    ]);
    
    if (!firstPass.success) {
      return {
        success: false,
        output: "",
        errors: [firstPass.error || "First pass compilation failed"],
        stats: {
          compilationTime: performance.now() - startTime,
          outputSize: 0,
          linesOfCode: 0
        }
      };
    }
    
    // Second pass: Compile intermediate with itself
    const secondPass = await this.runCommand(this.ruchyBinary, [
      "compile",
      "--self-hosted",
      "--target", "typescript", 
      "--output", join(this.workDir, "output", outputFile),
      join(this.workDir, "output", outputFile + ".intermediate.ts")
    ]);
    
    const compilationTime = performance.now() - startTime;
    
    if (!secondPass.success) {
      return {
        success: false,
        output: "",
        errors: [secondPass.error || "Second pass compilation failed"],
        stats: {
          compilationTime,
          outputSize: 0,
          linesOfCode: 0
        }
      };
    }
    
    // Read output file and calculate stats
    const outputPath = join(this.workDir, "output", outputFile);
    const outputContent = await Deno.readTextFile(outputPath);
    const outputStats = await Deno.stat(outputPath);
    
    return {
      success: true,
      output: outputContent,
      errors: [],
      stats: {
        compilationTime,
        outputSize: outputStats.size,
        linesOfCode: outputContent.split("\n").length
      }
    };
  }
  
  /**
   * Compile with reference compiler for comparison
   */
  async compileReference(
    sourceFile: string,
    outputFile: string
  ): Promise<CompilationResult> {
    const startTime = performance.now();
    
    const result = await this.runCommand(this.ruchyBinary, [
      "compile",
      "--target", "typescript",
      "--output", join(this.workDir, "reference", outputFile),
      sourceFile
    ]);
    
    const compilationTime = performance.now() - startTime;
    
    if (!result.success) {
      return {
        success: false,
        output: "",
        errors: [result.error || "Reference compilation failed"],
        stats: {
          compilationTime,
          outputSize: 0,
          linesOfCode: 0
        }
      };
    }
    
    const outputPath = join(this.workDir, "reference", outputFile);
    const outputContent = await Deno.readTextFile(outputPath);
    const outputStats = await Deno.stat(outputPath);
    
    return {
      success: true,
      output: outputContent,
      errors: [],
      stats: {
        compilationTime,
        outputSize: outputStats.size,
        linesOfCode: outputContent.split("\n").length
      }
    };
  }
  
  /**
   * Differential testing: Compare self-compiled vs reference output
   */
  async differentialTest(
    selfCompiled: CompilationResult,
    reference: CompilationResult
  ): Promise<ValidationResult> {
    const differences: string[] = [];
    
    // Bit-for-bit comparison
    const bitIdentical = selfCompiled.output === reference.output;
    
    if (!bitIdentical) {
      // Find specific differences
      const selfLines = selfCompiled.output.split("\n");
      const refLines = reference.output.split("\n");
      
      for (let i = 0; i < Math.max(selfLines.length, refLines.length); i++) {
        if (selfLines[i] !== refLines[i]) {
          differences.push(
            `Line ${i + 1}: Self: "${selfLines[i] || ""}" vs Ref: "${refLines[i] || ""}"`
          );
          if (differences.length >= 10) {
            differences.push("... (more differences truncated)");
            break;
          }
        }
      }
    }
    
    // Semantic equivalence check (simplified - would need AST comparison)
    const semanticEquivalent = await this.checkSemanticEquivalence(
      selfCompiled.output,
      reference.output
    );
    
    // Performance comparison
    const performanceRatio = 
      selfCompiled.stats.compilationTime / reference.stats.compilationTime;
    
    return {
      pass: bitIdentical || semanticEquivalent,
      differences,
      metrics: {
        bitIdentical,
        semanticEquivalent,
        performanceRatio
      }
    };
  }
  
  /**
   * Check semantic equivalence of two TypeScript outputs
   */
  private async checkSemanticEquivalence(
    output1: string,
    output2: string
  ): Promise<boolean> {
    // Normalize whitespace and formatting
    const normalized1 = this.normalizeCode(output1);
    const normalized2 = this.normalizeCode(output2);
    
    // If normalized versions match, they're semantically equivalent
    if (normalized1 === normalized2) {
      return true;
    }
    
    // More sophisticated checks would involve:
    // - AST comparison
    // - Running both and comparing behavior
    // - Type checking both
    
    return false;
  }
  
  /**
   * Normalize code for semantic comparison
   */
  private normalizeCode(code: string): string {
    return code
      .replace(/\/\/.*$/gm, "") // Remove comments
      .replace(/\/\*[\s\S]*?\*\//g, "") // Remove block comments
      .replace(/\s+/g, " ") // Normalize whitespace
      .replace(/;\s*/g, ";") // Normalize semicolons
      .replace(/{\s*/g, "{") // Normalize braces
      .replace(/}\s*/g, "}") // Normalize braces
      .trim();
  }
  
  /**
   * Run a command and capture output
   */
  private async runCommand(
    command: string,
    args: string[]
  ): Promise<{ success: boolean; output: string; error?: string }> {
    try {
      const process = new Deno.Command(command, {
        args,
        stdout: "piped",
        stderr: "piped"
      });
      
      const { code, stdout, stderr } = await process.output();
      
      if (code === 0) {
        return {
          success: true,
          output: new TextDecoder().decode(stdout)
        };
      } else {
        return {
          success: false,
          output: new TextDecoder().decode(stdout),
          error: new TextDecoder().decode(stderr)
        };
      }
    } catch (error) {
      return {
        success: false,
        output: "",
        error: error.message
      };
    }
  }
  
  /**
   * Generate performance report
   */
  generateReport(results: Map<string, ValidationResult>): string {
    let report = "# Self-Compilation Validation Report\n\n";
    report += `Generated: ${new Date().toISOString()}\n\n`;
    
    let totalTests = 0;
    let passedTests = 0;
    let bitIdenticalCount = 0;
    let totalPerformanceRatio = 0;
    
    for (const [testName, result] of results) {
      totalTests++;
      if (result.pass) passedTests++;
      if (result.metrics.bitIdentical) bitIdenticalCount++;
      totalPerformanceRatio += result.metrics.performanceRatio;
      
      report += `## Test: ${testName}\n`;
      report += `- Status: ${result.pass ? "✅ PASS" : "❌ FAIL"}\n`;
      report += `- Bit Identical: ${result.metrics.bitIdentical ? "Yes" : "No"}\n`;
      report += `- Semantic Equivalent: ${result.metrics.semanticEquivalent ? "Yes" : "No"}\n`;
      report += `- Performance Ratio: ${result.metrics.performanceRatio.toFixed(2)}x\n`;
      
      if (result.differences.length > 0) {
        report += `- Differences:\n`;
        for (const diff of result.differences.slice(0, 5)) {
          report += `  - ${diff}\n`;
        }
      }
      report += "\n";
    }
    
    report += "## Summary\n";
    report += `- Total Tests: ${totalTests}\n`;
    report += `- Passed: ${passedTests} (${((passedTests/totalTests)*100).toFixed(1)}%)\n`;
    report += `- Bit Identical: ${bitIdenticalCount} (${((bitIdenticalCount/totalTests)*100).toFixed(1)}%)\n`;
    report += `- Avg Performance Ratio: ${(totalPerformanceRatio/totalTests).toFixed(2)}x\n`;
    
    return report;
  }
}

/**
 * Test runner for self-compilation validation
 */
export async function runSelfCompilationTests(): Promise<void> {
  const harness = new SelfCompilationHarness();
  await harness.initialize();
  
  const results = new Map<string, ValidationResult>();
  
  // Test cases - compile various Ruchy components with themselves
  const testCases = [
    { name: "lexer", source: "bootstrap/stage0/lexer.ruchy" },
    { name: "parser", source: "bootstrap/stage1/parser.ruchy" },
    { name: "type_checker", source: "bootstrap/stage2/type_checker.ruchy" },
    { name: "code_generator", source: "bootstrap/stage3/code_generator.ruchy" }
  ];
  
  for (const testCase of testCases) {
    console.log(`\n🔧 Testing self-compilation: ${testCase.name}`);
    
    // Compile with self-hosted compiler
    const selfCompiled = await harness.compileSelfHosted(
      testCase.source,
      `${testCase.name}_self.ts`
    );
    
    // Compile with reference compiler
    const reference = await harness.compileReference(
      testCase.source,
      `${testCase.name}_ref.ts`
    );
    
    // Run differential test
    const result = await harness.differentialTest(selfCompiled, reference);
    results.set(testCase.name, result);
    
    console.log(`  Result: ${result.pass ? "✅ PASS" : "❌ FAIL"}`);
    console.log(`  Bit Identical: ${result.metrics.bitIdentical}`);
    console.log(`  Performance: ${result.metrics.performanceRatio.toFixed(2)}x`);
  }
  
  // Generate and save report
  const report = harness.generateReport(results);
  await Deno.writeTextFile(
    join(harness["workDir"], "self_compilation_report.md"),
    report
  );
  
  console.log("\n📊 Report saved to: build/validation/self_compilation_report.md");
  
  // Exit with error if any tests failed
  const allPassed = Array.from(results.values()).every(r => r.pass);
  if (!allPassed) {
    console.error("\n❌ Some self-compilation tests failed!");
    Deno.exit(1);
  }
  
  console.log("\n✅ All self-compilation tests passed!");
}

// Run tests if this is the main module
if (import.meta.main) {
  await runSelfCompilationTests();
}