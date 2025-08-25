#!/usr/bin/env -S deno run --allow-all

/**
 * VALID-001: Continuous Validation Pipeline
 * 
 * Automated pipeline for continuous validation of Ruchy self-compilation
 * Runs on every commit/change to ensure bootstrap stability
 */

import { join } from "https://deno.land/std@0.208.0/path/mod.ts";
import { ensureDir } from "https://deno.land/std@0.208.0/fs/mod.ts";
import { format } from "https://deno.land/std@0.208.0/datetime/mod.ts";
import { SelfCompilationHarness } from "./self_compilation_harness.ts";
import { DifferentialTestRunner } from "./differential_test_runner.ts";
import { OutputComparator } from "./output_comparator.ts";

interface PipelineConfig {
  watchMode: boolean;
  watchPaths: string[];
  outputDir: string;
  notifyOnFailure: boolean;
  generateReports: boolean;
  runInterval?: number; // milliseconds
}

interface PipelineResult {
  timestamp: Date;
  duration: number; // milliseconds
  testsRun: number;
  testsPassed: number;
  testsFailed: number;
  performanceMetrics: PerformanceMetrics;
  regressions: Regression[];
}

interface PerformanceMetrics {
  avgCompilationTime: number;
  avgPerformanceRatio: number;
  memoryUsage: number;
  throughput: number; // LOC/s
}

interface Regression {
  test: string;
  metric: string;
  previousValue: number;
  currentValue: number;
  percentChange: number;
}

/**
 * Continuous Validation Pipeline
 * Monitors changes and automatically validates self-compilation
 */
export class ContinuousValidationPipeline {
  private config: PipelineConfig;
  private harness: SelfCompilationHarness;
  private differentialRunner: DifferentialTestRunner;
  private comparator: OutputComparator;
  private lastResults: Map<string, any>;
  private isRunning: boolean;
  
  constructor(config: Partial<PipelineConfig> = {}) {
    this.config = {
      watchMode: false,
      watchPaths: ["bootstrap/", "src/", "tests/"],
      outputDir: "./build/continuous",
      notifyOnFailure: true,
      generateReports: true,
      runInterval: 60000, // 1 minute default
      ...config
    };
    
    this.harness = new SelfCompilationHarness(join(this.config.outputDir, "validation"));
    this.differentialRunner = new DifferentialTestRunner(join(this.config.outputDir, "differential"));
    this.comparator = new OutputComparator();
    this.lastResults = new Map();
    this.isRunning = false;
  }
  
  /**
   * Initialize the pipeline
   */
  async initialize(): Promise<void> {
    await ensureDir(this.config.outputDir);
    await ensureDir(join(this.config.outputDir, "reports"));
    await ensureDir(join(this.config.outputDir, "metrics"));
    await ensureDir(join(this.config.outputDir, "artifacts"));
    
    await this.harness.initialize();
    await this.differentialRunner.initialize();
    
    console.log("✅ Continuous validation pipeline initialized");
    console.log(`📁 Output directory: ${this.config.outputDir}`);
    console.log(`👁️ Watch mode: ${this.config.watchMode ? "ON" : "OFF"}`);
  }
  
  /**
   * Start the continuous validation pipeline
   */
  async start(): Promise<void> {
    console.log("\n🚀 Starting continuous validation pipeline");
    this.isRunning = true;
    
    if (this.config.watchMode) {
      await this.startWatchMode();
    } else {
      await this.runOnce();
    }
  }
  
  /**
   * Run validation once
   */
  async runOnce(): Promise<PipelineResult> {
    console.log("\n" + "=".repeat(60));
    console.log(`VALIDATION RUN - ${format(new Date(), "yyyy-MM-dd HH:mm:ss")}`);
    console.log("=".repeat(60));
    
    const startTime = performance.now();
    const result: PipelineResult = {
      timestamp: new Date(),
      duration: 0,
      testsRun: 0,
      testsPassed: 0,
      testsFailed: 0,
      performanceMetrics: {
        avgCompilationTime: 0,
        avgPerformanceRatio: 0,
        memoryUsage: 0,
        throughput: 0
      },
      regressions: []
    };
    
    try {
      // Run all validation tests
      const testResults = await this.runAllTests();
      
      // Analyze results
      result.testsRun = testResults.length;
      result.testsPassed = testResults.filter(r => r.passed).length;
      result.testsFailed = result.testsRun - result.testsPassed;
      
      // Calculate performance metrics
      result.performanceMetrics = await this.calculateMetrics(testResults);
      
      // Check for regressions
      result.regressions = this.detectRegressions(testResults);
      
      // Duration
      result.duration = performance.now() - startTime;
      
      // Generate reports if configured
      if (this.config.generateReports) {
        await this.generateReports(result, testResults);
      }
      
      // Store results for regression detection
      this.storeResults(testResults);
      
      // Notify if failures
      if (this.config.notifyOnFailure && result.testsFailed > 0) {
        this.notifyFailures(result);
      }
      
      // Print summary
      this.printSummary(result);
      
    } catch (error) {
      console.error(`Pipeline error: ${error.message}`);
      result.testsFailed = result.testsRun || 1;
    }
    
    return result;
  }
  
  /**
   * Run all validation tests
   */
  private async runAllTests(): Promise<any[]> {
    const tests = [];
    
    console.log("\n📋 Running validation tests...");
    
    // Test 1: Lexer self-compilation
    console.log("\n1️⃣ Testing lexer self-compilation");
    const lexerTest = await this.testSelfCompilation(
      "bootstrap/stage0/lexer.ruchy",
      "lexer"
    );
    tests.push(lexerTest);
    
    // Test 2: Parser self-compilation
    console.log("\n2️⃣ Testing parser self-compilation");
    const parserTest = await this.testSelfCompilation(
      "bootstrap/stage1/parser.ruchy",
      "parser"
    );
    tests.push(parserTest);
    
    // Test 3: Type checker self-compilation
    console.log("\n3️⃣ Testing type checker self-compilation");
    const typeTest = await this.testSelfCompilation(
      "bootstrap/stage2/type_checker.ruchy",
      "type_checker"
    );
    tests.push(typeTest);
    
    // Test 4: Code generator self-compilation
    console.log("\n4️⃣ Testing code generator self-compilation");
    const codegenTest = await this.testSelfCompilation(
      "bootstrap/stage3/code_generator.ruchy",
      "code_generator"
    );
    tests.push(codegenTest);
    
    return tests;
  }
  
  /**
   * Test self-compilation of a component
   */
  private async testSelfCompilation(sourceFile: string, name: string): Promise<any> {
    const startTime = performance.now();
    
    try {
      // Compile with self-hosted compiler
      const selfCompiled = await this.harness.compileSelfHosted(
        sourceFile,
        `${name}_self.ts`
      );
      
      // Compile with reference compiler
      const reference = await this.harness.compileReference(
        sourceFile,
        `${name}_ref.ts`
      );
      
      // Compare outputs
      const comparison = await this.comparator.compareOutputs(
        selfCompiled.output,
        reference.output,
        "semantic"
      );
      
      const compilationTime = performance.now() - startTime;
      
      return {
        name,
        passed: comparison.identical || comparison.category === "semantic",
        compilationTime,
        performanceRatio: selfCompiled.stats.compilationTime / reference.stats.compilationTime,
        comparison,
        selfCompiled,
        reference
      };
    } catch (error) {
      return {
        name,
        passed: false,
        error: error.message,
        compilationTime: performance.now() - startTime
      };
    }
  }
  
  /**
   * Calculate performance metrics
   */
  private async calculateMetrics(testResults: any[]): Promise<PerformanceMetrics> {
    const compilationTimes = testResults.map(r => r.compilationTime || 0);
    const performanceRatios = testResults.map(r => r.performanceRatio || 1);
    
    const avgCompilationTime = compilationTimes.reduce((a, b) => a + b, 0) / compilationTimes.length;
    const avgPerformanceRatio = performanceRatios.reduce((a, b) => a + b, 0) / performanceRatios.length;
    
    // Estimate memory usage
    const memInfo = Deno.memoryUsage();
    const memoryUsage = memInfo.rss / (1024 * 1024); // MB
    
    // Calculate throughput
    const totalLOC = testResults.reduce((sum, r) => 
      sum + (r.selfCompiled?.stats?.linesOfCode || 0), 0);
    const totalTime = compilationTimes.reduce((a, b) => a + b, 0) / 1000; // seconds
    const throughput = totalLOC / totalTime;
    
    return {
      avgCompilationTime,
      avgPerformanceRatio,
      memoryUsage,
      throughput
    };
  }
  
  /**
   * Detect performance regressions
   */
  private detectRegressions(testResults: any[]): Regression[] {
    const regressions: Regression[] = [];
    
    for (const result of testResults) {
      const lastResult = this.lastResults.get(result.name);
      
      if (lastResult) {
        // Check compilation time regression
        if (result.compilationTime > lastResult.compilationTime * 1.1) {
          regressions.push({
            test: result.name,
            metric: "compilation_time",
            previousValue: lastResult.compilationTime,
            currentValue: result.compilationTime,
            percentChange: ((result.compilationTime - lastResult.compilationTime) / lastResult.compilationTime) * 100
          });
        }
        
        // Check performance ratio regression
        if (result.performanceRatio > lastResult.performanceRatio * 1.1) {
          regressions.push({
            test: result.name,
            metric: "performance_ratio",
            previousValue: lastResult.performanceRatio,
            currentValue: result.performanceRatio,
            percentChange: ((result.performanceRatio - lastResult.performanceRatio) / lastResult.performanceRatio) * 100
          });
        }
      }
    }
    
    return regressions;
  }
  
  /**
   * Store results for future regression detection
   */
  private storeResults(testResults: any[]): void {
    for (const result of testResults) {
      this.lastResults.set(result.name, {
        compilationTime: result.compilationTime,
        performanceRatio: result.performanceRatio,
        timestamp: new Date()
      });
    }
  }
  
  /**
   * Generate reports
   */
  private async generateReports(result: PipelineResult, testResults: any[]): Promise<void> {
    const timestamp = format(result.timestamp, "yyyyMMdd_HHmmss");
    const reportDir = join(this.config.outputDir, "reports", timestamp);
    await ensureDir(reportDir);
    
    // Main report
    const mainReport = this.generateMainReport(result, testResults);
    await Deno.writeTextFile(join(reportDir, "validation_report.md"), mainReport);
    
    // Performance report
    const perfReport = this.generatePerformanceReport(result);
    await Deno.writeTextFile(join(reportDir, "performance_report.md"), perfReport);
    
    // Regression report if any
    if (result.regressions.length > 0) {
      const regressionReport = this.generateRegressionReport(result.regressions);
      await Deno.writeTextFile(join(reportDir, "regression_report.md"), regressionReport);
    }
    
    console.log(`\n📊 Reports saved to: ${reportDir}`);
  }
  
  /**
   * Generate main validation report
   */
  private generateMainReport(result: PipelineResult, testResults: any[]): string {
    let report = "# Continuous Validation Report\n\n";
    report += `**Timestamp**: ${format(result.timestamp, "yyyy-MM-dd HH:mm:ss")}\n`;
    report += `**Duration**: ${(result.duration / 1000).toFixed(2)}s\n\n`;
    
    report += "## Summary\n\n";
    report += `- **Tests Run**: ${result.testsRun}\n`;
    report += `- **Passed**: ${result.testsPassed} ✅\n`;
    report += `- **Failed**: ${result.testsFailed} ❌\n`;
    report += `- **Success Rate**: ${((result.testsPassed / result.testsRun) * 100).toFixed(1)}%\n\n`;
    
    report += "## Test Results\n\n";
    for (const test of testResults) {
      report += `### ${test.name}\n`;
      report += `- **Status**: ${test.passed ? "✅ PASS" : "❌ FAIL"}\n`;
      report += `- **Compilation Time**: ${test.compilationTime.toFixed(2)}ms\n`;
      report += `- **Performance Ratio**: ${test.performanceRatio?.toFixed(2) || "N/A"}\n`;
      if (test.comparison) {
        report += `- **Output Similarity**: ${test.comparison.similarity.toFixed(1)}%\n`;
        report += `- **Category**: ${test.comparison.category}\n`;
      }
      if (test.error) {
        report += `- **Error**: ${test.error}\n`;
      }
      report += "\n";
    }
    
    return report;
  }
  
  /**
   * Generate performance report
   */
  private generatePerformanceReport(result: PipelineResult): string {
    let report = "# Performance Report\n\n";
    report += `**Timestamp**: ${format(result.timestamp, "yyyy-MM-dd HH:mm:ss")}\n\n`;
    
    report += "## Metrics\n\n";
    report += `- **Avg Compilation Time**: ${result.performanceMetrics.avgCompilationTime.toFixed(2)}ms\n`;
    report += `- **Avg Performance Ratio**: ${result.performanceMetrics.avgPerformanceRatio.toFixed(2)}x\n`;
    report += `- **Memory Usage**: ${result.performanceMetrics.memoryUsage.toFixed(2)}MB\n`;
    report += `- **Throughput**: ${result.performanceMetrics.throughput.toFixed(0)} LOC/s\n\n`;
    
    return report;
  }
  
  /**
   * Generate regression report
   */
  private generateRegressionReport(regressions: Regression[]): string {
    let report = "# ⚠️ Performance Regression Report\n\n";
    
    for (const reg of regressions) {
      report += `## ${reg.test} - ${reg.metric}\n`;
      report += `- **Previous**: ${reg.previousValue.toFixed(2)}\n`;
      report += `- **Current**: ${reg.currentValue.toFixed(2)}\n`;
      report += `- **Change**: +${reg.percentChange.toFixed(1)}% ⚠️\n\n`;
    }
    
    return report;
  }
  
  /**
   * Print summary to console
   */
  private printSummary(result: PipelineResult): void {
    console.log("\n" + "=".repeat(60));
    console.log("VALIDATION SUMMARY");
    console.log("=".repeat(60));
    console.log(`Tests: ${result.testsPassed}/${result.testsRun} passed`);
    console.log(`Duration: ${(result.duration / 1000).toFixed(2)}s`);
    console.log(`Throughput: ${result.performanceMetrics.throughput.toFixed(0)} LOC/s`);
    
    if (result.regressions.length > 0) {
      console.log(`\n⚠️ ${result.regressions.length} performance regressions detected!`);
    }
    
    if (result.testsFailed === 0) {
      console.log("\n✅ All validation tests passed!");
    } else {
      console.log(`\n❌ ${result.testsFailed} tests failed`);
    }
  }
  
  /**
   * Notify about failures
   */
  private notifyFailures(result: PipelineResult): void {
    console.log("\n🔔 VALIDATION FAILURE NOTIFICATION");
    console.log(`${result.testsFailed} tests failed at ${format(result.timestamp, "HH:mm:ss")}`);
    // In a real system, this could send emails, Slack messages, etc.
  }
  
  /**
   * Start watch mode
   */
  private async startWatchMode(): Promise<void> {
    console.log("👁️ Watch mode enabled - monitoring for changes...");
    
    // Run initial validation
    await this.runOnce();
    
    // Set up periodic runs
    setInterval(async () => {
      if (this.isRunning) {
        console.log("\n⏰ Scheduled validation run");
        await this.runOnce();
      }
    }, this.config.runInterval!);
    
    // Keep process alive
    await new Promise(() => {});
  }
  
  /**
   * Stop the pipeline
   */
  stop(): void {
    console.log("\n🛑 Stopping continuous validation pipeline");
    this.isRunning = false;
  }
}

/**
 * CLI interface for the pipeline
 */
if (import.meta.main) {
  const pipeline = new ContinuousValidationPipeline({
    watchMode: Deno.args.includes("--watch"),
    generateReports: !Deno.args.includes("--no-reports"),
    notifyOnFailure: !Deno.args.includes("--quiet")
  });
  
  await pipeline.initialize();
  await pipeline.start();
}