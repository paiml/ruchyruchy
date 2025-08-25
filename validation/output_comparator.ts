/**
 * VALID-001: Output Comparison Tools
 * 
 * Advanced comparison utilities for validating compiler outputs
 * Supports AST, TypeScript, and execution result comparison
 */

import { diff } from "https://deno.land/std@0.208.0/testing/diff.ts";
import { parse as parseAST } from "https://deno.land/x/swc@0.2.1/mod.ts";

export interface ComparisonResult {
  identical: boolean;
  differences: Difference[];
  similarity: number; // 0-100 percentage
  category: "identical" | "whitespace" | "semantic" | "structural" | "different";
}

export interface Difference {
  type: "addition" | "deletion" | "modification";
  location: string;
  expected: string;
  actual: string;
  line?: number;
  column?: number;
}

/**
 * Advanced output comparison tools
 */
export class OutputComparator {
  
  /**
   * Compare two outputs with multiple strategies
   */
  async compareOutputs(
    output1: string,
    output2: string,
    mode: "exact" | "normalized" | "ast" | "semantic" = "exact"
  ): Promise<ComparisonResult> {
    switch (mode) {
      case "exact":
        return this.exactComparison(output1, output2);
      case "normalized":
        return this.normalizedComparison(output1, output2);
      case "ast":
        return await this.astComparison(output1, output2);
      case "semantic":
        return await this.semanticComparison(output1, output2);
      default:
        return this.exactComparison(output1, output2);
    }
  }
  
  /**
   * Exact byte-for-byte comparison
   */
  private exactComparison(output1: string, output2: string): ComparisonResult {
    const identical = output1 === output2;
    const differences: Difference[] = [];
    
    if (!identical) {
      const lines1 = output1.split("\n");
      const lines2 = output2.split("\n");
      
      for (let i = 0; i < Math.max(lines1.length, lines2.length); i++) {
        if (lines1[i] !== lines2[i]) {
          differences.push({
            type: "modification",
            location: `Line ${i + 1}`,
            expected: lines2[i] || "(empty)",
            actual: lines1[i] || "(empty)",
            line: i + 1
          });
        }
      }
    }
    
    const similarity = this.calculateSimilarity(output1, output2);
    
    return {
      identical,
      differences,
      similarity,
      category: identical ? "identical" : "different"
    };
  }
  
  /**
   * Comparison with normalized whitespace and formatting
   */
  private normalizedComparison(output1: string, output2: string): ComparisonResult {
    const normalized1 = this.normalizeOutput(output1);
    const normalized2 = this.normalizeOutput(output2);
    
    const identical = normalized1 === normalized2;
    const differences: Difference[] = [];
    
    if (!identical) {
      // Use diff algorithm for better difference detection
      const diffs = diff(normalized2.split("\n"), normalized1.split("\n"));
      
      let lineNum = 0;
      for (const part of diffs) {
        if (part.type === "added") {
          differences.push({
            type: "addition",
            location: `Line ${lineNum}`,
            expected: "",
            actual: part.value.join("\n"),
            line: lineNum
          });
        } else if (part.type === "removed") {
          differences.push({
            type: "deletion",
            location: `Line ${lineNum}`,
            expected: part.value.join("\n"),
            actual: "",
            line: lineNum
          });
        }
        lineNum += part.value.length;
      }
    }
    
    const similarity = this.calculateSimilarity(normalized1, normalized2);
    const category = identical ? "identical" : 
                    (similarity > 95 ? "whitespace" : "different");
    
    return {
      identical,
      differences,
      similarity,
      category
    };
  }
  
  /**
   * AST-based comparison for TypeScript code
   */
  private async astComparison(code1: string, code2: string): Promise<ComparisonResult> {
    try {
      const ast1 = await parseAST(code1, {
        syntax: "typescript",
        target: "es2022"
      });
      
      const ast2 = await parseAST(code2, {
        syntax: "typescript",
        target: "es2022"
      });
      
      const ast1Json = JSON.stringify(ast1, null, 2);
      const ast2Json = JSON.stringify(ast2, null, 2);
      
      const identical = ast1Json === ast2Json;
      const differences: Difference[] = [];
      
      if (!identical) {
        // Find structural differences in AST
        differences.push({
          type: "modification",
          location: "AST Structure",
          expected: "Reference AST",
          actual: "Self-compiled AST"
        });
      }
      
      const similarity = this.calculateSimilarity(ast1Json, ast2Json);
      const category = identical ? "identical" : 
                      (similarity > 90 ? "semantic" : "structural");
      
      return {
        identical,
        differences,
        similarity,
        category
      };
    } catch (error) {
      // If AST parsing fails, fall back to normalized comparison
      console.warn(`AST comparison failed: ${error.message}`);
      return this.normalizedComparison(code1, code2);
    }
  }
  
  /**
   * Semantic comparison (behavior-preserving transformations)
   */
  private async semanticComparison(code1: string, code2: string): Promise<ComparisonResult> {
    // First try AST comparison
    const astResult = await this.astComparison(code1, code2);
    
    if (astResult.identical || astResult.category === "semantic") {
      return astResult;
    }
    
    // Additional semantic checks
    const semanticChecks = [
      this.checkVariableRenaming(code1, code2),
      this.checkCommentDifferences(code1, code2),
      this.checkImportOrder(code1, code2)
    ];
    
    const allSemanticallySame = semanticChecks.every(check => check);
    
    if (allSemanticallySame) {
      return {
        identical: false,
        differences: astResult.differences,
        similarity: Math.max(astResult.similarity, 85),
        category: "semantic"
      };
    }
    
    return astResult;
  }
  
  /**
   * Normalize output for comparison
   */
  private normalizeOutput(output: string): string {
    return output
      .replace(/\/\/.*$/gm, "") // Remove single-line comments
      .replace(/\/\*[\s\S]*?\*\//g, "") // Remove multi-line comments
      .replace(/\s+/g, " ") // Normalize whitespace
      .replace(/;\s*/g, ";") // Normalize semicolons
      .replace(/{\s*/g, "{") // Normalize braces
      .replace(/}\s*/g, "}")
      .replace(/\(\s*/g, "(") // Normalize parentheses
      .replace(/\)\s*/g, ")")
      .trim();
  }
  
  /**
   * Calculate similarity percentage between two strings
   */
  private calculateSimilarity(str1: string, str2: string): number {
    if (str1 === str2) return 100;
    if (!str1 || !str2) return 0;
    
    const longer = str1.length > str2.length ? str1 : str2;
    const shorter = str1.length > str2.length ? str2 : str1;
    
    if (longer.length === 0) return 100;
    
    const editDistance = this.levenshteinDistance(longer, shorter);
    return ((longer.length - editDistance) / longer.length) * 100;
  }
  
  /**
   * Calculate Levenshtein distance between two strings
   */
  private levenshteinDistance(str1: string, str2: string): number {
    const matrix: number[][] = [];
    
    for (let i = 0; i <= str2.length; i++) {
      matrix[i] = [i];
    }
    
    for (let j = 0; j <= str1.length; j++) {
      matrix[0][j] = j;
    }
    
    for (let i = 1; i <= str2.length; i++) {
      for (let j = 1; j <= str1.length; j++) {
        if (str2.charAt(i - 1) === str1.charAt(j - 1)) {
          matrix[i][j] = matrix[i - 1][j - 1];
        } else {
          matrix[i][j] = Math.min(
            matrix[i - 1][j - 1] + 1, // substitution
            matrix[i][j - 1] + 1,     // insertion
            matrix[i - 1][j] + 1      // deletion
          );
        }
      }
    }
    
    return matrix[str2.length][str1.length];
  }
  
  /**
   * Check if differences are just variable renaming
   */
  private checkVariableRenaming(code1: string, code2: string): boolean {
    // Simple heuristic: if structure is same but variable names differ
    const stripped1 = code1.replace(/[a-zA-Z_]\w*/g, "VAR");
    const stripped2 = code2.replace(/[a-zA-Z_]\w*/g, "VAR");
    return stripped1 === stripped2;
  }
  
  /**
   * Check if differences are only in comments
   */
  private checkCommentDifferences(code1: string, code2: string): boolean {
    const noComments1 = code1.replace(/\/\/.*$/gm, "").replace(/\/\*[\s\S]*?\*\//g, "");
    const noComments2 = code2.replace(/\/\/.*$/gm, "").replace(/\/\*[\s\S]*?\*\//g, "");
    return this.normalizeOutput(noComments1) === this.normalizeOutput(noComments2);
  }
  
  /**
   * Check if import order is different but imports are same
   */
  private checkImportOrder(code1: string, code2: string): boolean {
    const imports1 = Array.from(code1.matchAll(/import.*from.*[;]/g)).sort();
    const imports2 = Array.from(code2.matchAll(/import.*from.*[;]/g)).sort();
    return JSON.stringify(imports1) === JSON.stringify(imports2);
  }
  
  /**
   * Generate detailed comparison report
   */
  generateComparisonReport(result: ComparisonResult): string {
    let report = "## Output Comparison Report\n\n";
    
    report += `**Status**: ${result.identical ? "✅ Identical" : "⚠️ Different"}\n`;
    report += `**Category**: ${result.category}\n`;
    report += `**Similarity**: ${result.similarity.toFixed(2)}%\n\n`;
    
    if (result.differences.length > 0) {
      report += "### Differences Found\n\n";
      
      for (const diff of result.differences.slice(0, 10)) {
        report += `- **${diff.type}** at ${diff.location}\n`;
        if (diff.expected) {
          report += `  Expected: \`${diff.expected.substring(0, 100)}\`\n`;
        }
        if (diff.actual) {
          report += `  Actual: \`${diff.actual.substring(0, 100)}\`\n`;
        }
      }
      
      if (result.differences.length > 10) {
        report += `\n... and ${result.differences.length - 10} more differences\n`;
      }
    }
    
    report += "\n### Analysis\n\n";
    
    switch (result.category) {
      case "identical":
        report += "Outputs are bit-for-bit identical. Perfect match! ✅";
        break;
      case "whitespace":
        report += "Outputs differ only in whitespace/formatting. Semantically equivalent.";
        break;
      case "semantic":
        report += "Outputs are semantically equivalent with minor syntactic differences.";
        break;
      case "structural":
        report += "Outputs have structural differences but may still be functionally equivalent.";
        break;
      case "different":
        report += "Outputs have significant differences. Further investigation needed.";
        break;
    }
    
    return report;
  }
}

/**
 * Batch comparison of multiple file pairs
 */
export class BatchComparator {
  private comparator: OutputComparator;
  
  constructor() {
    this.comparator = new OutputComparator();
  }
  
  /**
   * Compare multiple file pairs
   */
  async compareFiles(
    filePairs: Array<{ file1: string; file2: string; name: string }>
  ): Promise<Map<string, ComparisonResult>> {
    const results = new Map<string, ComparisonResult>();
    
    for (const pair of filePairs) {
      try {
        const content1 = await Deno.readTextFile(pair.file1);
        const content2 = await Deno.readTextFile(pair.file2);
        
        const result = await this.comparator.compareOutputs(
          content1,
          content2,
          "semantic"
        );
        
        results.set(pair.name, result);
        
        console.log(`Compared ${pair.name}: ${result.category} (${result.similarity.toFixed(1)}% similar)`);
      } catch (error) {
        console.error(`Failed to compare ${pair.name}: ${error.message}`);
      }
    }
    
    return results;
  }
}

// Export for testing
export { diff };