use derive_getters::Getters;

pub use chrono::{DateTime, Duration, TimeZone, Utc};

/// A `TestSuite` groups together several [`TestCase`s](../struct.TestCase.html).
#[derive(Debug, Clone, Getters)]
pub struct TestSuite {
    pub name: String,
    pub package: String,
    pub timestamp: DateTime<Utc>,
    pub hostname: String,
    pub testcases: Vec<TestCase>,
}

impl TestSuite {
    /// Create a new `TestSuite` with a given name
    pub fn new(name: &str) -> TestSuite {
        TestSuite {
            hostname: "localhost".into(),
            package: format!("testsuite/{}", &name),
            name: name.into(),
            timestamp: Utc::now(),
            testcases: Vec::new(),
        }
    }

    /// Add a [`TestCase`](../struct.TestCase.html) to the `TestSuite`.
    pub fn add_testcase(&mut self, testcase: TestCase) {
        self.testcases.push(testcase);
    }

    /// Add several [`TestCase`s](../struct.TestCase.html) from a Vec.
    pub fn add_testcases(&mut self, testcases: impl IntoIterator<Item = TestCase>) {
        self.testcases.extend(testcases);
    }

    /// Set the timestamp of the given `TestSuite`.
    ///
    /// By default the timestamp is set to the time when the `TestSuite` was created.
    pub fn set_timestamp(&mut self, timestamp: DateTime<Utc>) {
        self.timestamp = timestamp;
    }

    pub fn tests(&self) -> usize {
        self.testcases.len()
    }

    pub fn errors(&self) -> usize {
        self.testcases.iter().filter(|x| x.is_error()).count()
    }

    pub fn failures(&self) -> usize {
        self.testcases.iter().filter(|x| x.is_failure()).count()
    }

    pub fn time(&self) -> Duration {
        self.testcases
            .iter()
            .fold(Duration::zero(), |sum, d| sum + d.time)
    }
}

/// One single test case
#[derive(Debug, Clone, Getters)]
pub struct TestCase {
    pub name: String,
    pub time: Duration,
    pub result: TestResult,
    pub classname: Option<String>,
}

/// Result of a test case
#[derive(Debug, Clone)]
pub enum TestResult {
    Success,
    Error { type_: String, message: String },
    Failure { type_: String, message: String },
}

impl TestCase {
    /// Creates a new successful `TestCase`
    pub fn success(name: &str, time: Duration, classname: Option<String>) -> TestCase {
        TestCase {
            name: name.into(),
            time,
            result: TestResult::Success,
            classname,
        }
    }

    /// Check if a `TestCase` is successful
    pub fn is_success(&self) -> bool {
        match self.result {
            TestResult::Success => true,
            _ => false,
        }
    }

    /// Creates a new erroneous `TestCase`
    ///
    /// An erroneous `TestCase` is one that encountered an unexpected error condition.
    pub fn error(
        name: &str,
        time: Duration,
        type_: &str,
        message: &str,
        classname: Option<String>,
    ) -> TestCase {
        TestCase {
            name: name.into(),
            time,
            result: TestResult::Error {
                type_: type_.into(),
                message: message.into(),
            },
            classname,
        }
    }

    /// Check if a `TestCase` is erroneous
    pub fn is_error(&self) -> bool {
        match self.result {
            TestResult::Error { .. } => true,
            _ => false,
        }
    }

    /// Creates a new failed `TestCase`
    ///
    /// A failed `TestCase` is one where an explicit assertion failed
    pub fn failure(
        name: &str,
        time: Duration,
        type_: &str,
        message: &str,
        classname: Option<String>,
    ) -> TestCase {
        TestCase {
            name: name.into(),
            time,
            result: TestResult::Failure {
                type_: type_.into(),
                message: message.into(),
            },
            classname,
        }
    }

    /// Check if a `TestCase` failed
    pub fn is_failure(&self) -> bool {
        match self.result {
            TestResult::Failure { .. } => true,
            _ => false,
        }
    }
}

// Make sure the readme is tested too
#[cfg(doctest)]
doc_comment::doctest!("../README.md");
