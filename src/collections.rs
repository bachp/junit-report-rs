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
    pub system_out: Option<String>,
    pub system_err: Option<String>,
}

impl TestSuite {
    /// Create a new `TestSuite` with a given name
    pub fn new(name: &str) -> Self {
        TestSuite {
            hostname: "localhost".into(),
            package: format!("testsuite/{}", &name),
            name: name.into(),
            timestamp: Utc::now(),
            testcases: Vec::new(),
            system_out: None,
            system_err: None,
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

    pub fn set_system_out(&mut self, system_out: &str) {
        self.system_out = Some(system_out.to_owned());
    }

    pub fn set_system_err(&mut self, system_err: &str) {
        self.system_err = Some(system_err.to_owned());
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

    pub fn skipped(&self) -> usize {
        self.testcases.iter().filter(|x| x.is_skipped()).count()
    }

    pub fn time(&self) -> Duration {
        self.testcases
            .iter()
            .fold(Duration::zero(), |sum, d| sum + d.time)
    }
}

/// A `TestSuite` groups together several [`TestCase`s](../struct.TestCase.html).
#[derive(Debug, Clone, Getters)]
pub struct TestSuiteBuilder {
    pub name: String,
    pub package: String,
    pub timestamp: DateTime<Utc>,
    pub hostname: String,
    pub testcases: Vec<TestCase>,
    pub system_out: Option<String>,
    pub system_err: Option<String>,
}

impl TestSuiteBuilder {
    /// Create a new `TestSuite` with a given name
    pub fn new(name: &str) -> Self {
        TestSuiteBuilder {
            hostname: "localhost".into(),
            package: format!("testsuite/{}", &name),
            name: name.into(),
            timestamp: Utc::now(),
            testcases: Vec::new(),
            system_out: None,
            system_err: None,
        }
    }

    /// Add a [`TestCase`](../struct.TestCase.html) to the `TestSuite`.
    pub fn add_testcase(&mut self, testcase: TestCase) -> &mut Self {
        self.testcases.push(testcase);
        self
    }

    /// Add several [`TestCase`s](../struct.TestCase.html) from a Vec.
    pub fn add_testcases(&mut self, testcases: impl IntoIterator<Item = TestCase>) -> &mut Self {
        self.testcases.extend(testcases);
        self
    }

    /// Set the timestamp of the given `TestSuite`.
    ///
    /// By default the timestamp is set to the time when the `TestSuite` was created.
    pub fn set_timestamp(&mut self, timestamp: DateTime<Utc>) -> &mut Self {
        self.timestamp = timestamp;
        self
    }

    pub fn set_system_out(&mut self, system_out: &str) -> &mut Self {
        self.system_out = Some(system_out.to_owned());
        self
    }

    pub fn set_system_err(&mut self, system_err: &str) -> &mut Self {
        self.system_err = Some(system_err.to_owned());
        self
    }

    pub fn build(&self) -> TestSuite {
        TestSuite {
            hostname: self.hostname.clone(),
            package: self.package.clone(),
            name: self.name.clone(),
            timestamp: self.timestamp.clone(),
            testcases: self.testcases.clone(),
            system_out: self.system_out.clone(),
            system_err: self.system_err.clone(),
        }
    }
}

/// One single test case
#[derive(Debug, Clone, Getters)]
pub struct TestCase {
    pub name: String,
    pub time: Duration,
    pub result: TestResult,
    pub classname: Option<String>,
    pub system_out: Option<String>,
    pub system_err: Option<String>,
}

/// Result of a test case
#[derive(Debug, Clone)]
pub enum TestResult {
    Success,
    Skipped,
    Error { type_: String, message: String },
    Failure { type_: String, message: String },
}

impl TestCase {
    /// Creates a new successful `TestCase`
    pub fn success(name: &str, time: Duration) -> Self {
        TestCase {
            name: name.into(),
            time,
            result: TestResult::Success,
            classname: None,
            system_out: None,
            system_err: None,
        }
    }

    /// Set the `classname` for the `TestCase`
    pub fn set_classname(&mut self, classname: &str) {
        self.classname = Some(classname.to_owned());
    }

    /// Set the `system_out` for the `TestCase`
    pub fn set_system_out(&mut self, system_out: &str) {
        self.system_out = Some(system_out.to_owned());
    }

    /// Set the `system_err` for the `TestCase`
    pub fn set_system_err(&mut self, system_err: &str) {
        self.system_err = Some(system_err.to_owned());
    }

    /// Check if a `TestCase` is successful
    pub fn is_success(&self) -> bool {
        matches!(self.result, TestResult::Success)
    }

    /// Creates a new erroneous `TestCase`
    ///
    /// An erroneous `TestCase` is one that encountered an unexpected error condition.
    pub fn error(name: &str, time: Duration, type_: &str, message: &str) -> Self {
        TestCase {
            name: name.into(),
            time,
            result: TestResult::Error {
                type_: type_.into(),
                message: message.into(),
            },
            classname: None,
            system_out: None,
            system_err: None,
        }
    }

    /// Check if a `TestCase` is erroneous
    pub fn is_error(&self) -> bool {
        matches!(self.result, TestResult::Error { .. })
    }

    /// Creates a new failed `TestCase`
    ///
    /// A failed `TestCase` is one where an explicit assertion failed
    pub fn failure(name: &str, time: Duration, type_: &str, message: &str) -> Self {
        TestCase {
            name: name.into(),
            time,
            result: TestResult::Failure {
                type_: type_.into(),
                message: message.into(),
            },
            classname: None,
            system_out: None,
            system_err: None,
        }
    }

    /// Check if a `TestCase` failed
    pub fn is_failure(&self) -> bool {
        matches!(self.result, TestResult::Failure { .. })
    }

    /// Create a new ignored `TestCase`
    ///
    /// An ignored `TestCase` is one where an ignored or skipped
    pub fn skipped(name: &str) -> Self {
        TestCase {
            name: name.into(),
            time: Duration::zero(),
            result: TestResult::Skipped,
            classname: None,
            system_out: None,
            system_err: None,
        }
    }

    /// Check if a `TestCase` ignored
    pub fn is_skipped(&self) -> bool {
        matches!(self.result, TestResult::Skipped)
    }
}

/// One single test case
#[derive(Debug, Clone, Getters)]
pub struct TestCaseBuilder {
    pub name: String,
    pub time: Duration,
    pub result: TestResult,
    pub classname: Option<String>,
    pub system_out: Option<String>,
    pub system_err: Option<String>,
}

impl TestCaseBuilder {
    /// Creates a new successful `TestCase`
    pub fn success(name: &str, time: Duration) -> Self {
        TestCaseBuilder {
            name: name.into(),
            time,
            result: TestResult::Success,
            classname: None,
            system_out: None,
            system_err: None,
        }
    }

    /// Set the `classname` for the `TestCase`
    pub fn set_classname(&mut self, classname: &str) -> &mut Self {
        self.classname = Some(classname.to_owned());
        self
    }

    /// Set the `system_out` for the `TestCase`
    pub fn set_system_out(&mut self, system_out: &str) -> &mut Self {
        self.system_out = Some(system_out.to_owned());
        self
    }

    /// Set the `system_err` for the `TestCase`
    pub fn set_system_err(&mut self, system_err: &str) -> &mut Self {
        self.system_err = Some(system_err.to_owned());
        self
    }

    /// Creates a new erroneous `TestCase`
    ///
    /// An erroneous `TestCase` is one that encountered an unexpected error condition.
    pub fn error(name: &str, time: Duration, type_: &str, message: &str) -> Self {
        TestCaseBuilder {
            name: name.into(),
            time,
            result: TestResult::Error {
                type_: type_.into(),
                message: message.into(),
            },
            classname: None,
            system_out: None,
            system_err: None,
        }
    }

    /// Creates a new failed `TestCase`
    ///
    /// A failed `TestCase` is one where an explicit assertion failed
    pub fn failure(name: &str, time: Duration, type_: &str, message: &str) -> Self {
        TestCaseBuilder {
            name: name.into(),
            time,
            result: TestResult::Failure {
                type_: type_.into(),
                message: message.into(),
            },
            classname: None,
            system_out: None,
            system_err: None,
        }
    }

    /// Create a new ignored `TestCase`
    ///
    /// An ignored `TestCase` is one where an ignored or skipped
    pub fn skipped(name: &str) -> Self {
        TestCaseBuilder {
            name: name.into(),
            time: Duration::zero(),
            result: TestResult::Skipped,
            classname: None,
            system_out: None,
            system_err: None,
        }
    }

    /// Creates a new successful `TestCase`
    pub fn build(&self) -> TestCase {
        TestCase {
            name: self.name.clone(),
            time: self.time.clone(),
            result: self.result.clone(),
            classname: self.classname.clone(),
            system_out: self.system_out.clone(),
            system_err: self.system_err.clone(),
        }
    }
}

// Make sure the readme is tested too
#[cfg(doctest)]
doc_comment::doctest!("../README.md");
