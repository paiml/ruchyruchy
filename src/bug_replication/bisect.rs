// Git Bisection Tool for Regression Narrowing
// REPLIC-003: Bisection Tool Implementation
//
// References:
// - Zeller (2009): "Why Programs Fail" - Chapter 13: Simplifying Failure-Inducing Input
// - Davies & Roper (2014): "Bug Localisation through Diverse Spectrum"
// - Section 7.3 of BUG_DISCOVERY_REPORTER_REPLICATOR_SPEC.md

use std::fmt;

/// Git commit identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommitId(pub String);

impl CommitId {
    /// Create a new commit ID
    pub fn new(id: String) -> Self {
        CommitId(id)
    }

    /// Get short hash (first 7 characters)
    pub fn short(&self) -> String {
        self.0.chars().take(7).collect()
    }
}

impl fmt::Display for CommitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.short())
    }
}

/// Commit metadata
#[derive(Debug, Clone)]
pub struct Commit {
    /// Commit identifier
    pub id: CommitId,
    /// Commit message
    pub message: String,
    /// Author name
    pub author: String,
    /// Timestamp (Unix epoch seconds)
    pub timestamp: u64,
}

impl Commit {
    /// Create a new commit
    pub fn new(id: CommitId, message: String, author: String, timestamp: u64) -> Self {
        Commit {
            id,
            message,
            author,
            timestamp,
        }
    }
}

/// Result of testing a commit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestResult {
    /// Test passed (bug not present)
    Good,
    /// Test failed (bug present)
    Bad,
    /// Test result unclear (build failure, timeout, etc.)
    Skip,
}

impl TestResult {
    /// Check if result is good
    pub fn is_good(&self) -> bool {
        matches!(self, TestResult::Good)
    }

    /// Check if result is bad
    pub fn is_bad(&self) -> bool {
        matches!(self, TestResult::Bad)
    }

    /// Check if result is skip
    pub fn is_skip(&self) -> bool {
        matches!(self, TestResult::Skip)
    }
}

/// Bisection state
#[derive(Debug, Clone)]
pub struct BisectionState {
    /// Known good commit
    pub good: CommitId,
    /// Known bad commit
    pub bad: CommitId,
    /// Commits tested so far
    pub tested: Vec<(CommitId, TestResult)>,
    /// Current commit being tested (if any)
    pub current: Option<CommitId>,
}

impl BisectionState {
    /// Create a new bisection state
    pub fn new(good: CommitId, bad: CommitId) -> Self {
        BisectionState {
            good,
            bad,
            tested: Vec::new(),
            current: None,
        }
    }

    /// Record test result for current commit
    pub fn record_result(&mut self, commit: CommitId, result: TestResult) {
        self.tested.push((commit.clone(), result));

        // Update good/bad boundaries based on result
        match result {
            TestResult::Good => {
                self.good = commit;
            }
            TestResult::Bad => {
                self.bad = commit;
            }
            TestResult::Skip => {
                // Keep current boundaries
            }
        }

        self.current = None;
    }

    /// Get number of commits tested
    pub fn commits_tested(&self) -> usize {
        self.tested.len()
    }

    /// Check if a commit has been tested
    pub fn has_tested(&self, commit: &CommitId) -> bool {
        self.tested.iter().any(|(c, _)| c == commit)
    }

    /// Get result for a commit if tested
    pub fn get_result(&self, commit: &CommitId) -> Option<TestResult> {
        self.tested
            .iter()
            .find(|(c, _)| c == commit)
            .map(|(_, r)| *r)
    }
}

/// Bisection result
#[derive(Debug, Clone)]
pub struct BisectionResult {
    /// First bad commit (regression)
    pub first_bad_commit: Commit,
    /// Last good commit (before regression)
    pub last_good_commit: Commit,
    /// Number of commits tested
    pub commits_tested: usize,
    /// All test results
    pub test_results: Vec<(CommitId, TestResult)>,
}

impl BisectionResult {
    /// Create a new bisection result
    pub fn new(
        first_bad_commit: Commit,
        last_good_commit: Commit,
        commits_tested: usize,
        test_results: Vec<(CommitId, TestResult)>,
    ) -> Self {
        BisectionResult {
            first_bad_commit,
            last_good_commit,
            commits_tested,
            test_results,
        }
    }

    /// Generate summary text
    pub fn summary(&self) -> String {
        format!(
            "Regression introduced in commit {} (\"{}\")\nLast good: {} (\"{}\")\nCommits tested: {}",
            self.first_bad_commit.id.short(),
            self.first_bad_commit.message,
            self.last_good_commit.id.short(),
            self.last_good_commit.message,
            self.commits_tested,
        )
    }
}

/// Git bisection tool
pub struct GitBisector<F>
where
    F: FnMut(&CommitId) -> TestResult,
{
    /// Test oracle function
    test_fn: F,
    /// Bisection state
    state: BisectionState,
    /// Commit history (ordered from old to new)
    commits: Vec<Commit>,
}

impl<F> GitBisector<F>
where
    F: FnMut(&CommitId) -> TestResult,
{
    /// Create a new git bisector
    pub fn new(
        test_fn: F,
        good: CommitId,
        bad: CommitId,
        commits: Vec<Commit>,
    ) -> Self {
        GitBisector {
            test_fn,
            state: BisectionState::new(good, bad),
            commits,
        }
    }

    /// Find the index of a commit in the history
    fn find_commit_index(&self, commit: &CommitId) -> Option<usize> {
        self.commits.iter().position(|c| &c.id == commit)
    }

    /// Get commit range between good and bad
    fn get_commit_range(&self) -> Vec<&Commit> {
        let good_idx = self.find_commit_index(&self.state.good);
        let bad_idx = self.find_commit_index(&self.state.bad);

        if let (Some(good), Some(bad)) = (good_idx, bad_idx) {
            if good < bad {
                self.commits[good + 1..=bad].iter().collect()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }

    /// Select next commit to test (binary search midpoint)
    fn select_next_commit(&self) -> Option<CommitId> {
        let range = self.get_commit_range();

        if range.is_empty() {
            return None;
        }

        // Find midpoint of untested commits
        let untested: Vec<&Commit> = range
            .into_iter()
            .filter(|c| !self.state.has_tested(&c.id))
            .collect();

        if untested.is_empty() {
            return None;
        }

        // Select middle commit
        let mid = untested.len() / 2;
        Some(untested[mid].id.clone())
    }

    /// Run bisection to find first bad commit
    pub fn bisect(&mut self) -> Option<BisectionResult> {
        // Verify initial good commit is actually good
        let good_result = (self.test_fn)(&self.state.good);
        self.state.record_result(self.state.good.clone(), good_result);

        if !good_result.is_good() {
            // Initial "good" commit is not actually good
            return None;
        }

        // Verify initial bad commit is actually bad
        let bad_result = (self.test_fn)(&self.state.bad);
        self.state.record_result(self.state.bad.clone(), bad_result);

        if !bad_result.is_bad() {
            // Initial "bad" commit is not actually bad
            return None;
        }

        // Binary search for first bad commit
        loop {
            let next = self.select_next_commit();

            match next {
                Some(commit) => {
                    self.state.current = Some(commit.clone());
                    let result = (self.test_fn)(&commit);
                    self.state.record_result(commit, result);
                }
                None => {
                    // No more commits to test - found first bad commit
                    break;
                }
            }
        }

        // Get final result
        let bad_idx = self.find_commit_index(&self.state.bad)?;
        let good_idx = self.find_commit_index(&self.state.good)?;

        if bad_idx == 0 || good_idx >= self.commits.len() {
            return None;
        }

        let first_bad = self.commits[bad_idx].clone();
        let last_good = self.commits[good_idx].clone();

        Some(BisectionResult::new(
            first_bad,
            last_good,
            self.state.commits_tested(),
            self.state.tested.clone(),
        ))
    }

    /// Get current bisection state
    pub fn state(&self) -> &BisectionState {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_commits(count: usize) -> Vec<Commit> {
        (0..count)
            .map(|i| {
                Commit::new(
                    CommitId::new(format!("commit{:03}", i)),
                    format!("Message {}", i),
                    "Author".to_string(),
                    1000 + i as u64,
                )
            })
            .collect()
    }

    #[test]
    fn test_commit_id_creation() {
        let id = CommitId::new("abc123def456".to_string());
        assert_eq!(id.short(), "abc123d");
    }

    #[test]
    fn test_commit_id_display() {
        let id = CommitId::new("abc123def456".to_string());
        assert_eq!(format!("{}", id), "abc123d");
    }

    #[test]
    fn test_commit_creation() {
        let commit = Commit::new(
            CommitId::new("abc123".to_string()),
            "Test commit".to_string(),
            "Alice".to_string(),
            1234567890,
        );

        assert_eq!(commit.id.0, "abc123");
        assert_eq!(commit.message, "Test commit");
        assert_eq!(commit.author, "Alice");
        assert_eq!(commit.timestamp, 1234567890);
    }

    #[test]
    fn test_test_result_checks() {
        assert!(TestResult::Good.is_good());
        assert!(!TestResult::Good.is_bad());
        assert!(!TestResult::Good.is_skip());

        assert!(TestResult::Bad.is_bad());
        assert!(!TestResult::Bad.is_good());

        assert!(TestResult::Skip.is_skip());
    }

    #[test]
    fn test_bisection_state_creation() {
        let good = CommitId::new("good123".to_string());
        let bad = CommitId::new("bad456".to_string());

        let state = BisectionState::new(good.clone(), bad.clone());

        assert_eq!(state.good, good);
        assert_eq!(state.bad, bad);
        assert_eq!(state.commits_tested(), 0);
        assert!(state.current.is_none());
    }

    #[test]
    fn test_bisection_state_record_result() {
        let good = CommitId::new("good123".to_string());
        let bad = CommitId::new("bad456".to_string());
        let mut state = BisectionState::new(good, bad);

        let test_commit = CommitId::new("test789".to_string());
        state.record_result(test_commit.clone(), TestResult::Good);

        assert_eq!(state.commits_tested(), 1);
        assert!(state.has_tested(&test_commit));
        assert_eq!(state.get_result(&test_commit), Some(TestResult::Good));
    }

    #[test]
    fn test_bisection_result_summary() {
        let first_bad = Commit::new(
            CommitId::new("bad123".to_string()),
            "Broke feature".to_string(),
            "Alice".to_string(),
            2000,
        );

        let last_good = Commit::new(
            CommitId::new("good456".to_string()),
            "Working".to_string(),
            "Bob".to_string(),
            1000,
        );

        let result = BisectionResult::new(first_bad, last_good, 5, vec![]);

        let summary = result.summary();
        assert!(summary.contains("bad123"));
        assert!(summary.contains("Broke feature"));
        assert!(summary.contains("good456"));
        assert!(summary.contains("Commits tested: 5"));
    }

    #[test]
    fn test_git_bisector_creation() {
        let commits = create_test_commits(10);
        let test_fn = |_: &CommitId| TestResult::Good;

        let bisector = GitBisector::new(
            test_fn,
            CommitId::new("commit000".to_string()),
            CommitId::new("commit009".to_string()),
            commits,
        );

        assert_eq!(bisector.state.commits_tested(), 0);
    }

    #[test]
    fn test_git_bisector_find_commit() {
        let commits = create_test_commits(10);
        let test_fn = |_: &CommitId| TestResult::Good;

        let bisector = GitBisector::new(
            test_fn,
            CommitId::new("commit000".to_string()),
            CommitId::new("commit009".to_string()),
            commits,
        );

        let idx = bisector.find_commit_index(&CommitId::new("commit005".to_string()));
        assert_eq!(idx, Some(5));
    }

    #[test]
    fn test_git_bisector_commit_range() {
        let commits = create_test_commits(10);
        let test_fn = |_: &CommitId| TestResult::Good;

        let bisector = GitBisector::new(
            test_fn,
            CommitId::new("commit002".to_string()),
            CommitId::new("commit007".to_string()),
            commits,
        );

        let range = bisector.get_commit_range();
        assert_eq!(range.len(), 5); // commits 3, 4, 5, 6, 7
    }

    #[test]
    fn test_git_bisector_simple_case() {
        let commits = create_test_commits(10);

        // Bug introduced at commit 5
        let test_fn = |commit: &CommitId| {
            let commit_num: usize = commit.0[6..9].parse().unwrap();
            if commit_num < 5 {
                TestResult::Good
            } else {
                TestResult::Bad
            }
        };

        let mut bisector = GitBisector::new(
            test_fn,
            CommitId::new("commit000".to_string()),
            CommitId::new("commit009".to_string()),
            commits,
        );

        let result = bisector.bisect();
        assert!(result.is_some());

        let result = result.unwrap();
        assert_eq!(result.first_bad_commit.id.0, "commit005");
        assert_eq!(result.last_good_commit.id.0, "commit004");
    }

    #[test]
    fn test_git_bisector_multiple_commits() {
        let commits = create_test_commits(20);

        // Bug introduced at commit 12
        let test_fn = |commit: &CommitId| {
            let commit_num: usize = commit.0[6..9].parse().unwrap();
            if commit_num < 12 {
                TestResult::Good
            } else {
                TestResult::Bad
            }
        };

        let mut bisector = GitBisector::new(
            test_fn,
            CommitId::new("commit000".to_string()),
            CommitId::new("commit019".to_string()),
            commits,
        );

        let result = bisector.bisect();
        assert!(result.is_some());

        let result = result.unwrap();
        assert_eq!(result.first_bad_commit.id.0, "commit012");
        assert_eq!(result.last_good_commit.id.0, "commit011");

        // Binary search should take ~log2(20) = ~5 tests
        assert!(result.commits_tested <= 8);
    }

    #[test]
    fn test_git_bisector_invalid_initial_good() {
        let commits = create_test_commits(10);

        // All commits are bad
        let test_fn = |_: &CommitId| TestResult::Bad;

        let mut bisector = GitBisector::new(
            test_fn,
            CommitId::new("commit000".to_string()),
            CommitId::new("commit009".to_string()),
            commits,
        );

        let result = bisector.bisect();
        assert!(result.is_none()); // Cannot bisect if good commit is not good
    }

    #[test]
    fn test_git_bisector_invalid_initial_bad() {
        let commits = create_test_commits(10);

        // All commits are good
        let test_fn = |_: &CommitId| TestResult::Good;

        let mut bisector = GitBisector::new(
            test_fn,
            CommitId::new("commit000".to_string()),
            CommitId::new("commit009".to_string()),
            commits,
        );

        let result = bisector.bisect();
        assert!(result.is_none()); // Cannot bisect if bad commit is not bad
    }
}
