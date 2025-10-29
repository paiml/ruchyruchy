// Issue Linking & Deduplication Module
// Implements GITHUB-002 from specification v1.0.0
//
// Purpose: Intelligent issue linking and deduplication
// - Find related bugs by file, error message, symptom
// - Prevent duplicate issue creation
// - Calculate similarity scores for issues
//
// References:
// - Jaccard similarity: |A ∩ B| / |A ∪ B|
// - Levenshtein distance: Edit distance between strings
// - TF-IDF: Term frequency-inverse document frequency
// - Runeson et al. (2007): "Detection of Duplicate Defect Reports Using Natural Language Processing"
// - Sun et al. (2010): "A Discriminative Model Approach for Accurate Duplicate Bug Report Retrieval"

use std::collections::{HashMap, HashSet};

/// Similarity threshold for duplicate detection (0.0-1.0)
pub const DUPLICATE_THRESHOLD: f64 = 0.80;

/// Similarity threshold for related issue detection (0.0-1.0)
pub const RELATED_THRESHOLD: f64 = 0.50;

/// Bug issue for similarity comparison
#[derive(Debug, Clone)]
pub struct BugIssue {
    /// Issue ID
    pub id: u64,

    /// Issue title
    pub title: String,

    /// Issue body
    pub body: String,

    /// Related files
    pub files: Vec<String>,

    /// Error message (if any)
    pub error_message: Option<String>,

    /// Labels
    pub labels: Vec<String>,
}

impl BugIssue {
    /// Create new bug issue
    pub fn new(id: u64, title: String, body: String) -> Self {
        Self {
            id,
            title,
            body,
            files: Vec::new(),
            error_message: None,
            labels: Vec::new(),
        }
    }

    /// Add file
    pub fn add_file(&mut self, file: String) {
        self.files.push(file);
    }

    /// Set error message
    pub fn set_error_message(&mut self, error: String) {
        self.error_message = Some(error);
    }

    /// Add label
    pub fn add_label(&mut self, label: String) {
        self.labels.push(label);
    }

    /// Get all text for similarity comparison
    pub fn text(&self) -> String {
        let mut text = String::new();
        text.push_str(&self.title);
        text.push(' ');
        text.push_str(&self.body);
        if let Some(ref error) = self.error_message {
            text.push(' ');
            text.push_str(error);
        }
        text
    }

    /// Get tokens from text (word-based)
    pub fn tokens(&self) -> Vec<String> {
        self.text()
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
            .filter(|s| !s.is_empty() && s.len() > 2) // Filter short tokens
            .collect()
    }

    /// Get token set for Jaccard similarity
    pub fn token_set(&self) -> HashSet<String> {
        self.tokens().into_iter().collect()
    }
}

/// Similarity score between two issues
#[derive(Debug, Clone)]
pub struct SimilarityScore {
    /// Overall similarity (0.0-1.0)
    pub overall: f64,

    /// Title similarity (0.0-1.0)
    pub title_similarity: f64,

    /// Body similarity (0.0-1.0)
    pub body_similarity: f64,

    /// File overlap (0.0-1.0)
    pub file_overlap: f64,

    /// Error message similarity (0.0-1.0)
    pub error_similarity: f64,

    /// Label overlap (0.0-1.0)
    pub label_overlap: f64,
}

impl SimilarityScore {
    /// Create new similarity score
    pub fn new(
        title_similarity: f64,
        body_similarity: f64,
        file_overlap: f64,
        error_similarity: f64,
        label_overlap: f64,
    ) -> Self {
        // Weighted average: title 30%, body 25%, files 20%, error 15%, labels 10%
        let overall = title_similarity * 0.30
            + body_similarity * 0.25
            + file_overlap * 0.20
            + error_similarity * 0.15
            + label_overlap * 0.10;

        Self {
            overall,
            title_similarity,
            body_similarity,
            file_overlap,
            error_similarity,
            label_overlap,
        }
    }

    /// Check if issues are duplicates
    pub fn is_duplicate(&self) -> bool {
        self.overall >= DUPLICATE_THRESHOLD
    }

    /// Check if issues are related
    pub fn is_related(&self) -> bool {
        self.overall >= RELATED_THRESHOLD
    }
}

/// Similarity calculator
pub struct SimilarityCalculator;

impl SimilarityCalculator {
    /// Calculate similarity between two issues
    pub fn calculate(issue1: &BugIssue, issue2: &BugIssue) -> SimilarityScore {
        let title_sim = Self::jaccard_similarity(&issue1.title, &issue2.title);
        let body_sim = Self::jaccard_similarity(&issue1.body, &issue2.body);
        let file_overlap = Self::set_overlap(&issue1.files, &issue2.files);
        let error_sim = Self::error_similarity(issue1, issue2);
        let label_overlap = Self::set_overlap(&issue1.labels, &issue2.labels);

        SimilarityScore::new(title_sim, body_sim, file_overlap, error_sim, label_overlap)
    }

    /// Jaccard similarity between two texts
    pub fn jaccard_similarity(text1: &str, text2: &str) -> f64 {
        let tokens1: HashSet<String> = Self::tokenize(text1).into_iter().collect();
        let tokens2: HashSet<String> = Self::tokenize(text2).into_iter().collect();

        if tokens1.is_empty() && tokens2.is_empty() {
            return 1.0; // Both empty = identical
        }

        let intersection = tokens1.intersection(&tokens2).count();
        let union = tokens1.union(&tokens2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    /// Tokenize text
    fn tokenize(text: &str) -> Vec<String> {
        text.to_lowercase()
            .split_whitespace()
            .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
            .filter(|s| !s.is_empty() && s.len() > 2)
            .collect()
    }

    /// Calculate set overlap (Jaccard for sets)
    pub fn set_overlap<T: std::hash::Hash + Eq + Clone>(set1: &[T], set2: &[T]) -> f64 {
        if set1.is_empty() && set2.is_empty() {
            return 1.0;
        }

        let s1: HashSet<_> = set1.iter().collect();
        let s2: HashSet<_> = set2.iter().collect();

        let intersection = s1.intersection(&s2).count();
        let union = s1.union(&s2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    /// Calculate error message similarity
    fn error_similarity(issue1: &BugIssue, issue2: &BugIssue) -> f64 {
        match (&issue1.error_message, &issue2.error_message) {
            (Some(e1), Some(e2)) => Self::jaccard_similarity(e1, e2),
            (None, None) => 1.0, // Both have no error = match
            _ => 0.0,            // One has error, one doesn't = no match
        }
    }
}

/// Duplicate detection result
#[derive(Debug, Clone)]
pub struct DuplicateResult {
    /// Is this a duplicate?
    pub is_duplicate: bool,

    /// Duplicate of issue ID (if duplicate)
    pub duplicate_of: Option<u64>,

    /// Similarity score
    pub score: SimilarityScore,
}

/// Related issues result
#[derive(Debug, Clone)]
pub struct RelatedIssue {
    /// Related issue ID
    pub issue_id: u64,

    /// Similarity score
    pub score: SimilarityScore,
}

/// Issue deduplicator
pub struct IssueDeduplicator {
    /// Existing issues
    issues: Vec<BugIssue>,
}

impl IssueDeduplicator {
    /// Create new deduplicator
    pub fn new() -> Self {
        Self {
            issues: Vec::new(),
        }
    }

    /// Add existing issue
    pub fn add_issue(&mut self, issue: BugIssue) {
        self.issues.push(issue);
    }

    /// Check if new issue is duplicate
    pub fn check_duplicate(&self, new_issue: &BugIssue) -> DuplicateResult {
        let mut best_match: Option<(u64, SimilarityScore)> = None;

        for existing in &self.issues {
            let score = SimilarityCalculator::calculate(new_issue, existing);

            if score.is_duplicate() {
                match &best_match {
                    None => best_match = Some((existing.id, score)),
                    Some((_, best_score)) => {
                        if score.overall > best_score.overall {
                            best_match = Some((existing.id, score));
                        }
                    }
                }
            }
        }

        match best_match {
            Some((id, score)) => DuplicateResult {
                is_duplicate: true,
                duplicate_of: Some(id),
                score,
            },
            None => DuplicateResult {
                is_duplicate: false,
                duplicate_of: None,
                score: SimilarityScore::new(0.0, 0.0, 0.0, 0.0, 0.0),
            },
        }
    }

    /// Find related issues
    pub fn find_related(&self, new_issue: &BugIssue, limit: usize) -> Vec<RelatedIssue> {
        let mut related: Vec<RelatedIssue> = self
            .issues
            .iter()
            .map(|existing| {
                let score = SimilarityCalculator::calculate(new_issue, existing);
                RelatedIssue {
                    issue_id: existing.id,
                    score,
                }
            })
            .filter(|r| r.score.is_related())
            .collect();

        // Sort by similarity score (highest first)
        related.sort_by(|a, b| {
            b.score
                .overall
                .partial_cmp(&a.score.overall)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Limit results
        related.truncate(limit);

        related
    }

    /// Get number of issues
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }
}

impl Default for IssueDeduplicator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bug_issue_creation() {
        let issue = BugIssue::new(1, "Test bug".to_string(), "Description".to_string());

        assert_eq!(issue.id, 1);
        assert_eq!(issue.title, "Test bug");
        assert_eq!(issue.body, "Description");
        assert_eq!(issue.files.len(), 0);
        assert!(issue.error_message.is_none());
        assert_eq!(issue.labels.len(), 0);
    }

    #[test]
    fn test_bug_issue_add_file() {
        let mut issue = BugIssue::new(1, "Test".to_string(), "Desc".to_string());
        issue.add_file("parser.rs".to_string());
        issue.add_file("lexer.rs".to_string());

        assert_eq!(issue.files.len(), 2);
        assert_eq!(issue.files[0], "parser.rs");
    }

    #[test]
    fn test_bug_issue_set_error() {
        let mut issue = BugIssue::new(1, "Test".to_string(), "Desc".to_string());
        issue.set_error_message("Stack overflow".to_string());

        assert!(issue.error_message.is_some());
        assert_eq!(issue.error_message.unwrap(), "Stack overflow");
    }

    #[test]
    fn test_bug_issue_text() {
        let mut issue = BugIssue::new(1, "Test bug".to_string(), "Description here".to_string());
        issue.set_error_message("Error occurred".to_string());

        let text = issue.text();
        assert!(text.contains("Test bug"));
        assert!(text.contains("Description here"));
        assert!(text.contains("Error occurred"));
    }

    #[test]
    fn test_bug_issue_tokens() {
        let issue = BugIssue::new(
            1,
            "Parser crashes on input".to_string(),
            "The parser fails".to_string(),
        );

        let tokens = issue.tokens();
        assert!(tokens.contains(&"parser".to_string()));
        assert!(tokens.contains(&"crashes".to_string()));
        assert!(tokens.contains(&"input".to_string()));
        assert!(tokens.contains(&"fails".to_string()));
    }

    #[test]
    fn test_jaccard_similarity_identical() {
        let sim = SimilarityCalculator::jaccard_similarity(
            "parser crashes on input",
            "parser crashes on input",
        );

        assert!((sim - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_jaccard_similarity_different() {
        let sim = SimilarityCalculator::jaccard_similarity(
            "parser crashes on input",
            "lexer works correctly",
        );

        assert!(sim < 0.5);
    }

    #[test]
    fn test_jaccard_similarity_partial() {
        let sim = SimilarityCalculator::jaccard_similarity(
            "parser crashes on nested expressions",
            "parser fails on nested input",
        );

        // Should have some overlap (parser, nested)
        assert!(sim > 0.3 && sim < 0.8);
    }

    #[test]
    fn test_set_overlap_identical() {
        let set1 = vec!["parser.rs".to_string(), "lexer.rs".to_string()];
        let set2 = vec!["parser.rs".to_string(), "lexer.rs".to_string()];

        let overlap = SimilarityCalculator::set_overlap(&set1, &set2);
        assert!((overlap - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_set_overlap_partial() {
        let set1 = vec![
            "parser.rs".to_string(),
            "lexer.rs".to_string(),
            "ast.rs".to_string(),
        ];
        let set2 = vec!["parser.rs".to_string(), "lexer.rs".to_string()];

        let overlap = SimilarityCalculator::set_overlap(&set1, &set2);
        assert!((overlap - 0.666).abs() < 0.01); // 2/3 = 0.666
    }

    #[test]
    fn test_set_overlap_empty() {
        let set1: Vec<String> = vec![];
        let set2: Vec<String> = vec![];

        let overlap = SimilarityCalculator::set_overlap(&set1, &set2);
        assert!((overlap - 1.0).abs() < 0.01); // Both empty = match
    }

    #[test]
    fn test_similarity_score_calculation() {
        let score = SimilarityScore::new(0.9, 0.8, 0.7, 0.6, 0.5);

        // Weighted: 0.9*0.3 + 0.8*0.25 + 0.7*0.2 + 0.6*0.15 + 0.5*0.1
        // = 0.27 + 0.2 + 0.14 + 0.09 + 0.05 = 0.75
        assert!((score.overall - 0.75).abs() < 0.01);
    }

    #[test]
    fn test_similarity_score_is_duplicate() {
        // Need high scores to exceed 0.80 threshold
        // 0.9*0.3 + 0.9*0.25 + 0.85*0.2 + 0.8*0.15 + 0.8*0.1 = 0.27 + 0.225 + 0.17 + 0.12 + 0.08 = 0.865
        let score = SimilarityScore::new(0.9, 0.9, 0.85, 0.8, 0.8);
        assert!(score.is_duplicate()); // overall > 0.80
        assert!(score.overall >= DUPLICATE_THRESHOLD);
    }

    #[test]
    fn test_similarity_score_is_related() {
        // Need scores that result in >= 0.50
        // 0.6*0.3 + 0.6*0.25 + 0.5*0.2 + 0.4*0.15 + 0.3*0.1 = 0.18 + 0.15 + 0.1 + 0.06 + 0.03 = 0.52
        let score = SimilarityScore::new(0.6, 0.6, 0.5, 0.4, 0.3);
        assert!(score.is_related()); // overall >= 0.50
        assert!(score.overall >= RELATED_THRESHOLD);
    }

    #[test]
    fn test_similarity_calculator_identical_issues() {
        let issue1 = BugIssue::new(
            1,
            "Parser crashes".to_string(),
            "Stack overflow on nested expressions".to_string(),
        );
        let issue2 = BugIssue::new(
            2,
            "Parser crashes".to_string(),
            "Stack overflow on nested expressions".to_string(),
        );

        let score = SimilarityCalculator::calculate(&issue1, &issue2);
        assert!(score.overall > 0.9);
        assert!(score.is_duplicate());
    }

    #[test]
    fn test_deduplicator_creation() {
        let dedup = IssueDeduplicator::new();
        assert_eq!(dedup.issue_count(), 0);
    }

    #[test]
    fn test_deduplicator_add_issue() {
        let mut dedup = IssueDeduplicator::new();
        let issue = BugIssue::new(1, "Test".to_string(), "Description".to_string());

        dedup.add_issue(issue);
        assert_eq!(dedup.issue_count(), 1);
    }

    #[test]
    fn test_deduplicator_check_duplicate_found() {
        let mut dedup = IssueDeduplicator::new();

        let existing = BugIssue::new(
            1,
            "Parser crashes on nested expressions".to_string(),
            "Stack overflow error when parsing deeply nested code".to_string(),
        );
        dedup.add_issue(existing);

        let new_issue = BugIssue::new(
            2,
            "Parser crashes on nested expressions".to_string(),
            "Stack overflow error when parsing deeply nested code".to_string(),
        );

        let result = dedup.check_duplicate(&new_issue);
        assert!(result.is_duplicate);
        assert_eq!(result.duplicate_of, Some(1));
        assert!(result.score.overall > 0.8);
    }

    #[test]
    fn test_deduplicator_check_duplicate_not_found() {
        let mut dedup = IssueDeduplicator::new();

        let existing = BugIssue::new(
            1,
            "Parser crashes".to_string(),
            "Stack overflow error".to_string(),
        );
        dedup.add_issue(existing);

        let new_issue = BugIssue::new(
            2,
            "Lexer fails".to_string(),
            "Invalid token error".to_string(),
        );

        let result = dedup.check_duplicate(&new_issue);
        assert!(!result.is_duplicate);
        assert!(result.duplicate_of.is_none());
    }

    #[test]
    fn test_deduplicator_find_related() {
        let mut dedup = IssueDeduplicator::new();

        let issue1 = BugIssue::new(
            1,
            "Parser crashes on nested expressions".to_string(),
            "Stack overflow".to_string(),
        );
        let issue2 = BugIssue::new(
            2,
            "Parser fails on deep nesting".to_string(),
            "Recursion limit".to_string(),
        );
        let issue3 = BugIssue::new(3, "Lexer bug".to_string(), "Token error".to_string());

        dedup.add_issue(issue1);
        dedup.add_issue(issue2);
        dedup.add_issue(issue3);

        let new_issue = BugIssue::new(
            4,
            "Parser problem with nested code".to_string(),
            "Deep nesting causes issues".to_string(),
        );

        let related = dedup.find_related(&new_issue, 5);

        // Should find issue1 and issue2 as related (both about parser and nesting)
        assert!(related.len() >= 1);
        assert!(related.iter().any(|r| r.issue_id == 1 || r.issue_id == 2));
    }

    #[test]
    fn test_deduplicator_find_related_limit() {
        let mut dedup = IssueDeduplicator::new();

        for i in 1..=10 {
            let issue = BugIssue::new(
                i,
                format!("Parser issue {}", i),
                "Parser related problem".to_string(),
            );
            dedup.add_issue(issue);
        }

        let new_issue = BugIssue::new(
            100,
            "Parser problem".to_string(),
            "Parser related issue".to_string(),
        );

        let related = dedup.find_related(&new_issue, 3);
        assert_eq!(related.len(), 3); // Limit to 3
    }
}
