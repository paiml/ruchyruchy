// DISC-004: Code Churn Analysis (INTEGRATION TESTS)
//
// Tests for code churn analysis to identify high-risk files.
//
// Requirements (from roadmap):
// - Git history parsing
// - Churn metrics (commits, lines, bugs/commit)
// - Hot spot identification
// - Timeline visualization
//
// Expected behavior:
// - Track file changes from git history
// - Calculate churn metrics per file
// - Identify high-risk files (hotspots)
// - Classify risk levels (Critical/High/Medium/Low/Minimal)
// - Generate confidence scores
// - Sort files by risk
//
// Testing Strategy:
// - Test churn calculation with real-world scenarios
// - Test hotspot detection across multiple files
// - Test timeline analysis with temporal patterns
// - Test risk classification accuracy
// - Test edge cases (single change, no changes, massive churn)

use ruchyruchy::bug_discovery::code_churn::{
    ChurnAnalyzer, ChurnHotspot, ChurnMetrics, FileChange, RiskLevel,
};

/// Test: Basic Churn Calculation
///
/// This test verifies basic churn metric calculation:
/// - Single file with multiple commits
/// - Total lines added/deleted
/// - Total churn (added + deleted)
/// - Change frequency
/// - Churn rate (lines per change)
#[test]
fn test_basic_churn_calculation() {
    let mut analyzer = ChurnAnalyzer::new().with_window_days(30);

    // Add 3 commits to src/parser.rs over 30 days
    analyzer.add_change(FileChange::new(
        "src/parser.rs".to_string(),
        "commit1".to_string(),
        50,  // +50 lines
        20,  // -20 lines
        "alice".to_string(),
        1000,
    ));

    analyzer.add_change(FileChange::new(
        "src/parser.rs".to_string(),
        "commit2".to_string(),
        30,  // +30 lines
        10,  // -10 lines
        "bob".to_string(),
        2000,
    ));

    analyzer.add_change(FileChange::new(
        "src/parser.rs".to_string(),
        "commit3".to_string(),
        20,  // +20 lines
        5,   // -5 lines
        "alice".to_string(),
        3000,
    ));

    let metrics = analyzer.calculate_metrics();
    assert_eq!(metrics.len(), 1);

    let parser_metrics = &metrics[0];
    assert_eq!(parser_metrics.file_path, "src/parser.rs");
    assert_eq!(parser_metrics.total_changes, 3);
    assert_eq!(parser_metrics.total_lines_added, 100);
    assert_eq!(parser_metrics.total_lines_deleted, 35);
    assert_eq!(parser_metrics.total_churn, 135);
    assert_eq!(parser_metrics.unique_authors, 2);

    // 3 changes over 30 days = 0.1 changes/day
    assert!((parser_metrics.change_frequency - 0.1).abs() < 0.01);

    // 135 total churn / 3 changes = 45 lines/change
    assert!((parser_metrics.churn_rate - 45.0).abs() < 0.1);
}

/// Test: Multiple File Churn Analysis
///
/// This test verifies churn analysis across multiple files:
/// - Different files with different churn patterns
/// - Metrics calculated per file
/// - No cross-file contamination
#[test]
fn test_multiple_file_churn_analysis() {
    let mut analyzer = ChurnAnalyzer::new();

    // High-churn file: src/lexer.rs (5 commits, lots of churn)
    for i in 0..5 {
        analyzer.add_change(FileChange::new(
            "src/lexer.rs".to_string(),
            format!("lex_commit_{}", i),
            100,
            50,
            format!("author_{}", i % 2), // 2 authors
            i as u64 * 1000,
        ));
    }

    // Medium-churn file: src/ast.rs (3 commits, moderate churn)
    for i in 0..3 {
        analyzer.add_change(FileChange::new(
            "src/ast.rs".to_string(),
            format!("ast_commit_{}", i),
            40,
            20,
            "alice".to_string(), // 1 author
            i as u64 * 1000,
        ));
    }

    // Low-churn file: src/utils.rs (1 commit, minimal churn)
    analyzer.add_change(FileChange::new(
        "src/utils.rs".to_string(),
        "utils_commit_1".to_string(),
        10,
        5,
        "bob".to_string(),
        1000,
    ));

    let metrics = analyzer.calculate_metrics();
    assert_eq!(metrics.len(), 3);

    // Find each file's metrics
    let lexer = metrics.iter().find(|m| m.file_path == "src/lexer.rs").unwrap();
    let ast = metrics.iter().find(|m| m.file_path == "src/ast.rs").unwrap();
    let utils = metrics.iter().find(|m| m.file_path == "src/utils.rs").unwrap();

    // Verify lexer has highest churn
    assert!(lexer.total_churn > ast.total_churn);
    assert!(ast.total_churn > utils.total_churn);

    // Verify change counts
    assert_eq!(lexer.total_changes, 5);
    assert_eq!(ast.total_changes, 3);
    assert_eq!(utils.total_changes, 1);
}

/// Test: Hotspot Detection
///
/// This test verifies identification of high-risk files:
/// - Multiple files with varying risk levels
/// - Hotspot threshold filtering
/// - Risk score calculation
/// - Risk level classification
#[test]
fn test_hotspot_detection() {
    let mut analyzer = ChurnAnalyzer::new().with_window_days(10);

    // Critical risk file: massive churn, many authors, frequent changes
    for i in 0..10 {
        analyzer.add_change(FileChange::new(
            "critical_file.rs".to_string(),
            format!("c{}", i),
            200,
            100,
            format!("author_{}", i), // 10 different authors
            i as u64 * 1000,
        ));
    }

    // High risk file: high churn, multiple authors
    for i in 0..6 {
        analyzer.add_change(FileChange::new(
            "high_risk.rs".to_string(),
            format!("h{}", i),
            100,
            50,
            format!("author_{}", i % 3), // 3 authors
            i as u64 * 1000,
        ));
    }

    // Medium risk file: moderate churn
    for i in 0..3 {
        analyzer.add_change(FileChange::new(
            "medium_risk.rs".to_string(),
            format!("m{}", i),
            50,
            25,
            "alice".to_string(), // 1 author
            i as u64 * 1000,
        ));
    }

    // Low risk file: minimal churn
    analyzer.add_change(FileChange::new(
        "low_risk.rs".to_string(),
        "l1".to_string(),
        10,
        5,
        "bob".to_string(),
        1000,
    ));

    // Identify hotspots with minimum risk score of 0.3
    let hotspots = analyzer.identify_hotspots(0.3);

    // Should identify critical, high, and medium risk files
    assert!(hotspots.len() >= 2);

    // Find critical file
    let critical = hotspots.iter().find(|h| h.metrics.file_path == "critical_file.rs");
    assert!(critical.is_some());

    let critical = critical.unwrap();
    assert_eq!(critical.risk_level, RiskLevel::Critical);
    assert!(critical.metrics.risk_score() >= 0.8);

    // Verify high risk file is included
    let high = hotspots.iter().find(|h| h.metrics.file_path == "high_risk.rs");
    assert!(high.is_some());
}

/// Test: Risk Level Classification
///
/// This test verifies risk level classification accuracy:
/// - Critical: risk score >= 0.8
/// - High: risk score >= 0.6
/// - Medium: risk score >= 0.4
/// - Low: risk score >= 0.2
/// - Minimal: risk score < 0.2
#[test]
fn test_risk_level_classification() {
    // Test boundary conditions
    assert_eq!(RiskLevel::from_score(1.0), RiskLevel::Critical);
    assert_eq!(RiskLevel::from_score(0.8), RiskLevel::Critical);
    assert_eq!(RiskLevel::from_score(0.79), RiskLevel::High);
    assert_eq!(RiskLevel::from_score(0.6), RiskLevel::High);
    assert_eq!(RiskLevel::from_score(0.59), RiskLevel::Medium);
    assert_eq!(RiskLevel::from_score(0.4), RiskLevel::Medium);
    assert_eq!(RiskLevel::from_score(0.39), RiskLevel::Low);
    assert_eq!(RiskLevel::from_score(0.2), RiskLevel::Low);
    assert_eq!(RiskLevel::from_score(0.19), RiskLevel::Minimal);
    assert_eq!(RiskLevel::from_score(0.0), RiskLevel::Minimal);
}

/// Test: Top N Hotspots
///
/// This test verifies top-N hotspot selection:
/// - Sort by risk score descending
/// - Return only top N
/// - Highest risk first
#[test]
fn test_top_n_hotspots() {
    let mut analyzer = ChurnAnalyzer::new();

    // Add 5 files with different risk levels
    let files = vec![
        ("low.rs", 1, 10, 5),      // Low risk
        ("medium.rs", 3, 50, 25),  // Medium risk
        ("high.rs", 5, 100, 50),   // High risk
        ("higher.rs", 7, 150, 75), // Higher risk
        ("critical.rs", 10, 200, 100), // Critical risk
    ];

    for (file, commits, added, deleted) in files {
        for i in 0..commits {
            analyzer.add_change(FileChange::new(
                file.to_string(),
                format!("{}_{}", file, i),
                added,
                deleted,
                format!("author_{}", i),
                i as u64 * 1000,
            ));
        }
    }

    // Get top 3 hotspots
    let top3 = analyzer.top_hotspots(3);
    assert_eq!(top3.len(), 3);

    // Verify descending order by risk score
    assert!(top3[0].metrics.risk_score() >= top3[1].metrics.risk_score());
    assert!(top3[1].metrics.risk_score() >= top3[2].metrics.risk_score());

    // Verify highest risk file is first
    assert_eq!(top3[0].metrics.file_path, "critical.rs");
}

/// Test: Timeline Analysis
///
/// This test verifies temporal churn patterns:
/// - Changes over time
/// - Change frequency calculation
/// - Window-based analysis
#[test]
fn test_timeline_analysis() {
    let mut analyzer = ChurnAnalyzer::new().with_window_days(100);

    // Simulate 100 days of development
    // Early phase: 10 changes in first 20 days (0.5 changes/day)
    for i in 0..10 {
        analyzer.add_change(FileChange::new(
            "src/feature.rs".to_string(),
            format!("early_{}", i),
            50,
            25,
            "alice".to_string(),
            i as u64 * 86400, // 1 day = 86400 seconds
        ));
    }

    // Middle phase: 5 changes between day 20-50 (slower)
    for i in 0..5 {
        analyzer.add_change(FileChange::new(
            "src/feature.rs".to_string(),
            format!("middle_{}", i),
            30,
            15,
            "bob".to_string(),
            (20 + i * 6) as u64 * 86400, // Every 6 days
        ));
    }

    // Late phase: 15 changes in last 30 days (0.5 changes/day, reactivated)
    for i in 0..15 {
        analyzer.add_change(FileChange::new(
            "src/feature.rs".to_string(),
            format!("late_{}", i),
            40,
            20,
            "charlie".to_string(),
            (70 + i * 2) as u64 * 86400, // Every 2 days
        ));
    }

    let metrics = analyzer.calculate_metrics();
    assert_eq!(metrics.len(), 1);

    let feature = &metrics[0];

    // Total: 30 changes over 100 days = 0.3 changes/day
    assert_eq!(feature.total_changes, 30);
    assert!((feature.change_frequency - 0.3).abs() < 0.01);

    // Verify all 3 authors tracked
    assert_eq!(feature.unique_authors, 3);
}

/// Test: Author Coordination Overhead
///
/// This test verifies author tracking for coordination risk:
/// - Single author = lower risk
/// - Multiple authors = higher risk (coordination overhead)
/// - Risk increases with author count
#[test]
fn test_author_coordination_overhead() {
    // Test 1: Single author (low coordination risk)
    let mut single_author = ChurnAnalyzer::new();
    for i in 0..5 {
        single_author.add_change(FileChange::new(
            "solo.rs".to_string(),
            format!("c{}", i),
            50,
            25,
            "alice".to_string(), // Same author
            i as u64 * 1000,
        ));
    }

    // Test 2: Multiple authors (high coordination risk)
    let mut multi_author = ChurnAnalyzer::new();
    for i in 0..5 {
        multi_author.add_change(FileChange::new(
            "team.rs".to_string(),
            format!("c{}", i),
            50,
            25,
            format!("author_{}", i), // Different authors
            i as u64 * 1000,
        ));
    }

    let solo_metrics = single_author.calculate_metrics();
    let team_metrics = multi_author.calculate_metrics();

    // Same churn, but different author counts
    assert_eq!(solo_metrics[0].total_churn, team_metrics[0].total_churn);
    assert_eq!(solo_metrics[0].unique_authors, 1);
    assert_eq!(team_metrics[0].unique_authors, 5);

    // Team file should have higher risk due to coordination
    assert!(team_metrics[0].risk_score() > solo_metrics[0].risk_score());
}

/// Test: Edge Case - Single Change
///
/// This test verifies handling of files with single commit:
/// - Minimal risk
/// - Valid metrics
/// - No division by zero
#[test]
fn test_edge_case_single_change() {
    let mut analyzer = ChurnAnalyzer::new();

    analyzer.add_change(FileChange::new(
        "once.rs".to_string(),
        "initial_commit".to_string(),
        100,
        0,
        "alice".to_string(),
        1000,
    ));

    let metrics = analyzer.calculate_metrics();
    assert_eq!(metrics.len(), 1);

    let once = &metrics[0];
    assert_eq!(once.total_changes, 1);
    assert_eq!(once.total_churn, 100);
    assert_eq!(once.unique_authors, 1);

    // Should have minimal risk (single change)
    assert!(once.risk_score() < 0.2);
}

/// Test: Edge Case - No Changes
///
/// This test verifies handling of analyzer with no data:
/// - Empty metrics list
/// - No hotspots
/// - No crashes
#[test]
fn test_edge_case_no_changes() {
    let analyzer = ChurnAnalyzer::new();

    let metrics = analyzer.calculate_metrics();
    assert_eq!(metrics.len(), 0);

    let hotspots = analyzer.identify_hotspots(0.0);
    assert_eq!(hotspots.len(), 0);

    let top = analyzer.top_hotspots(10);
    assert_eq!(top.len(), 0);
}

/// Test: Edge Case - Massive Churn
///
/// This test verifies handling of extremely high churn:
/// - Very large line counts
/// - Risk score capping at 1.0
/// - No overflow
#[test]
fn test_edge_case_massive_churn() {
    let mut analyzer = ChurnAnalyzer::new();

    // Add commits with massive churn
    for i in 0..20 {
        analyzer.add_change(FileChange::new(
            "massive.rs".to_string(),
            format!("c{}", i),
            10000, // +10K lines
            5000,  // -5K lines
            format!("author_{}", i),
            i as u64 * 1000,
        ));
    }

    let metrics = analyzer.calculate_metrics();
    assert_eq!(metrics.len(), 1);

    let massive = &metrics[0];

    // Total churn: 20 * (10000 + 5000) = 300,000 lines
    assert_eq!(massive.total_churn, 300_000);

    // Risk score should be capped at 1.0 (or close to it)
    assert!(massive.risk_score() <= 1.0);
    assert!(massive.risk_score() >= 0.9); // Should be very high risk
}

/// Test: Confidence Score Generation
///
/// This test verifies confidence scoring for hotspots:
/// - Higher change count = higher confidence
/// - More evidence = higher confidence
/// - Confidence in range [0.0, 1.0]
#[test]
fn test_confidence_score_generation() {
    // Test 1: Low confidence (few changes)
    let low_changes = vec![
        FileChange::new(
            "low_conf.rs".to_string(),
            "c1".to_string(),
            50,
            25,
            "alice".to_string(),
            1000,
        ),
    ];
    let low_metrics = ChurnMetrics::from_changes("low_conf.rs".to_string(), &low_changes, 30);
    let low_hotspot = ChurnHotspot::new(low_metrics);

    // Test 2: High confidence (many changes, multiple authors)
    let high_changes: Vec<FileChange> = (0..15)
        .map(|i| {
            FileChange::new(
                "high_conf.rs".to_string(),
                format!("c{}", i),
                50,
                25,
                format!("author_{}", i % 5), // 5 authors
                i as u64 * 1000,
            )
        })
        .collect();
    let high_metrics = ChurnMetrics::from_changes("high_conf.rs".to_string(), &high_changes, 30);
    let high_hotspot = ChurnHotspot::new(high_metrics);

    // Verify confidence scores are valid
    assert!(low_hotspot.confidence.overall >= 0.0);
    assert!(low_hotspot.confidence.overall <= 1.0);
    assert!(high_hotspot.confidence.overall >= 0.0);
    assert!(high_hotspot.confidence.overall <= 1.0);

    // High confidence hotspot should have higher score
    assert!(high_hotspot.confidence.overall > low_hotspot.confidence.overall);
}

/// Test: Churn Rate Calculation
///
/// This test verifies churn rate (lines changed per commit):
/// - High churn rate = large commits
/// - Low churn rate = small commits
/// - Accurate calculation
#[test]
fn test_churn_rate_calculation() {
    // Test 1: Large commits (high churn rate)
    let mut large_commits = ChurnAnalyzer::new();
    for i in 0..5 {
        large_commits.add_change(FileChange::new(
            "large.rs".to_string(),
            format!("c{}", i),
            100, // Large changes
            50,
            "alice".to_string(),
            i as u64 * 1000,
        ));
    }

    // Test 2: Small commits (low churn rate)
    let mut small_commits = ChurnAnalyzer::new();
    for i in 0..5 {
        small_commits.add_change(FileChange::new(
            "small.rs".to_string(),
            format!("c{}", i),
            10, // Small changes
            5,
            "bob".to_string(),
            i as u64 * 1000,
        ));
    }

    let large_metrics = large_commits.calculate_metrics();
    let small_metrics = small_commits.calculate_metrics();

    // Large commits: 5 * (100 + 50) = 750 / 5 = 150 lines/commit
    assert!((large_metrics[0].churn_rate - 150.0).abs() < 0.1);

    // Small commits: 5 * (10 + 5) = 75 / 5 = 15 lines/commit
    assert!((small_metrics[0].churn_rate - 15.0).abs() < 0.1);

    // Large commits should have higher risk
    assert!(large_metrics[0].risk_score() > small_metrics[0].risk_score());
}

/// Test: File Extension Patterns
///
/// This test verifies churn analysis works with different file types:
/// - Source files (.rs)
/// - Test files (.rs)
/// - Config files (.toml, .yaml)
/// - Documentation (.md)
#[test]
fn test_file_extension_patterns() {
    let mut analyzer = ChurnAnalyzer::new();

    let file_types = vec![
        "src/main.rs",
        "tests/test_feature.rs",
        "Cargo.toml",
        "config.yaml",
        "README.md",
    ];

    for file in &file_types {
        analyzer.add_change(FileChange::new(
            file.to_string(),
            format!("{}_commit", file),
            50,
            25,
            "alice".to_string(),
            1000,
        ));
    }

    let metrics = analyzer.calculate_metrics();
    assert_eq!(metrics.len(), 5);

    // Verify all file types tracked
    for file in &file_types {
        assert!(metrics.iter().any(|m| m.file_path == *file));
    }
}

/// Test: Real-World Scenario - Refactoring
///
/// This test simulates a refactoring scenario:
/// - Initial implementation
/// - Multiple refactoring commits
/// - Bug fixes
/// - Code stabilization
#[test]
fn test_real_world_refactoring_scenario() {
    let mut analyzer = ChurnAnalyzer::new().with_window_days(60);

    // Day 0-10: Initial implementation (high churn)
    for i in 0..5 {
        analyzer.add_change(FileChange::new(
            "src/module.rs".to_string(),
            format!("initial_{}", i),
            200,
            50,
            "alice".to_string(),
            i as u64 * 86400,
        ));
    }

    // Day 10-20: Refactoring (very high churn)
    for i in 0..8 {
        analyzer.add_change(FileChange::new(
            "src/module.rs".to_string(),
            format!("refactor_{}", i),
            150,
            150, // Lots of deletions + additions
            "bob".to_string(),
            (10 + i) as u64 * 86400,
        ));
    }

    // Day 20-40: Bug fixes (moderate churn)
    for i in 0..4 {
        analyzer.add_change(FileChange::new(
            "src/module.rs".to_string(),
            format!("bugfix_{}", i),
            30,
            20,
            "charlie".to_string(),
            (20 + i * 5) as u64 * 86400,
        ));
    }

    // Day 40-60: Stabilization (low churn)
    for i in 0..2 {
        analyzer.add_change(FileChange::new(
            "src/module.rs".to_string(),
            format!("polish_{}", i),
            10,
            5,
            "alice".to_string(),
            (40 + i * 10) as u64 * 86400,
        ));
    }

    let metrics = analyzer.calculate_metrics();
    assert_eq!(metrics.len(), 1);

    let module = &metrics[0];

    // Total: 19 commits over 60 days
    assert_eq!(module.total_changes, 19);

    // All 3 authors involved
    assert_eq!(module.unique_authors, 3);

    // Should be classified as high risk due to heavy refactoring
    assert!(module.risk_score() > 0.5);
    let risk_level = RiskLevel::from_score(module.risk_score());
    assert!(
        risk_level == RiskLevel::High || risk_level == RiskLevel::Critical,
        "Refactored file should be high or critical risk"
    );
}
