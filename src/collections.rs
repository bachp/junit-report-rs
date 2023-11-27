/*
 * Copyright (c) 2018 Pascal Bach
 * Copyright (c) 2021 Siemens Mobility GmbH
 *
 * SPDX-License-Identifier:     MIT
 */

use derive_getters::Getters;
use time::{Duration, OffsetDateTime};

/// A `TestSuite` groups together several [`TestCase`s](struct.TestCase.html).
#[derive(Debug, Clone, Getters)]
pub struct TestSuite {
    pub name: String,
    pub package: String,
    pub timestamp: OffsetDateTime,
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
            timestamp: OffsetDateTime::now_utc(),
            testcases: Vec::new(),
            system_out: None,
            system_err: None,
        }
    }

    /// Add a [`TestCase`](struct.TestCase.html) to the `TestSuite`.
    pub fn add_testcase(&mut self, testcase: TestCase) {
        self.testcases.push(testcase);
    }

    /// Add several [`TestCase`s](struct.TestCase.html) from a Vec.
    pub fn add_testcases(&mut self, testcases: impl IntoIterator<Item = TestCase>) {
        self.testcases.extend(testcases);
    }

    /// Set the timestamp of the given `TestSuite`.
    ///
    /// By default the timestamp is set to the time when the `TestSuite` was created.
    pub fn set_timestamp(&mut self, timestamp: OffsetDateTime) {
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
            .fold(Duration::ZERO, |sum, d| sum + d.time)
    }
}

///  Builder for [`TestSuite`](struct.TestSuite.html) objects.
#[derive(Debug, Clone, Getters)]
pub struct TestSuiteBuilder {
    pub testsuite: TestSuite,
}

impl TestSuiteBuilder {
    /// Create a new `TestSuiteBuilder` with a given name
    pub fn new(name: &str) -> Self {
        TestSuiteBuilder {
            testsuite: TestSuite::new(name),
        }
    }

    /// Add a [`TestCase`](struct.TestCase.html) to the `TestSuiteBuilder`.
    pub fn add_testcase(&mut self, testcase: TestCase) -> &mut Self {
        self.testsuite.testcases.push(testcase);
        self
    }

    /// Add several [`TestCase`s](struct.TestCase.html) from a Vec.
    pub fn add_testcases(&mut self, testcases: impl IntoIterator<Item = TestCase>) -> &mut Self {
        self.testsuite.testcases.extend(testcases);
        self
    }

    /// Set the timestamp of the `TestSuiteBuilder`.
    ///
    /// By default the timestamp is set to the time when the `TestSuiteBuilder` was created.
    pub fn set_timestamp(&mut self, timestamp: OffsetDateTime) -> &mut Self {
        self.testsuite.timestamp = timestamp;
        self
    }

    pub fn set_system_out(&mut self, system_out: &str) -> &mut Self {
        self.testsuite.system_out = Some(system_out.to_owned());
        self
    }

    pub fn set_system_err(&mut self, system_err: &str) -> &mut Self {
        self.testsuite.system_err = Some(system_err.to_owned());
        self
    }

    /// Build and return a [`TestSuite`](struct.TestSuite.html) object based on the data stored in this TestSuiteBuilder object.
    pub fn build(&self) -> TestSuite {
        self.testsuite.clone()
    }
}

/// One single test case
#[derive(Debug, Clone, Getters)]
pub struct TestCase {
    pub name: String,
    pub time: Duration,
    pub result: TestResult,
    pub classname: Option<String>,
    pub filepath: Option<String>,
    pub system_out: Option<String>,
    pub system_err: Option<String>,
}

/// Result of a test case
#[derive(Debug, Clone)]
pub enum TestResult {
    Success,
    Skipped,
    Error {
        type_: String,
        message: String,
        cause: Option<String>,
    },
    Failure {
        type_: String,
        message: String,
        cause: Option<String>,
    },
}

impl TestCase {
    /// Creates a new successful `TestCase`
    pub fn success(name: &str, time: Duration) -> Self {
        TestCase {
            name: name.into(),
            time,
            result: TestResult::Success,
            classname: None,
            filepath: None,
            system_out: None,
            system_err: None,
        }
    }

    /// Set the `classname` for the `TestCase`
    pub fn set_classname(&mut self, classname: &str) {
        self.classname = Some(classname.to_owned());
    }

    /// Set the `file` for the `TestCase`
    pub fn set_filepath(&mut self, filepath: &str) {
        self.filepath = Some(filepath.to_owned());
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
                cause: None,
            },
            classname: None,
            filepath: None,
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
                cause: None,
            },
            classname: None,
            filepath: None,
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
            time: Duration::ZERO,
            result: TestResult::Skipped,
            classname: None,
            filepath: None,
            system_out: None,
            system_err: None,
        }
    }

    /// Check if a `TestCase` ignored
    pub fn is_skipped(&self) -> bool {
        matches!(self.result, TestResult::Skipped)
    }
}

///  Builder for [`TestCase`](struct.TestCase.html) objects.
#[derive(Debug, Clone, Getters)]
pub struct TestCaseBuilder {
    pub testcase: TestCase,
}

impl TestCaseBuilder {
    /// Creates a new TestCaseBuilder for a successful `TestCase`
    pub fn success(name: &str, time: Duration) -> Self {
        TestCaseBuilder {
            testcase: TestCase::success(name, time),
        }
    }

    /// Set the `classname` for the `TestCase`
    pub fn set_classname(&mut self, classname: &str) -> &mut Self {
        self.testcase.classname = Some(classname.to_owned());
        self
    }

    /// Set the `file` for the `TestCase`
    pub fn set_filepath(&mut self, filepath: &str) -> &mut Self {
        self.testcase.filepath = Some(filepath.to_owned());
        self
    }

    /// Set the `system_out` for the `TestCase`
    pub fn set_system_out(&mut self, system_out: &str) -> &mut Self {
        self.testcase.system_out = Some(system_out.to_owned());
        self
    }

    /// Set the `system_err` for the `TestCase`
    pub fn set_system_err(&mut self, system_err: &str) -> &mut Self {
        self.testcase.system_err = Some(system_err.to_owned());
        self
    }

    /// Set the `result.trace` for the `TestCase`
    ///
    /// It has no effect on successful `TestCase`s.
    pub fn set_trace(&mut self, trace: &str) -> &mut Self {
        match self.testcase.result {
            TestResult::Error { ref mut cause, .. } => *cause = Some(trace.to_owned()),
            TestResult::Failure { ref mut cause, .. } => *cause = Some(trace.to_owned()),
            _ => {}
        }
        self
    }

    /// Creates a new TestCaseBuilder for an erroneous `TestCase`
    ///
    /// An erroneous `TestCase` is one that encountered an unexpected error condition.
    pub fn error(name: &str, time: Duration, type_: &str, message: &str) -> Self {
        TestCaseBuilder {
            testcase: TestCase::error(name, time, type_, message),
        }
    }

    /// Creates a new TestCaseBuilder for a failed `TestCase`
    ///
    /// A failed `TestCase` is one where an explicit assertion failed
    pub fn failure(name: &str, time: Duration, type_: &str, message: &str) -> Self {
        TestCaseBuilder {
            testcase: TestCase::failure(name, time, type_, message),
        }
    }

    /// Creates a new TestCaseBuilder for an ignored `TestCase`
    ///
    /// An ignored `TestCase` is one where an ignored or skipped
    pub fn skipped(name: &str) -> Self {
        TestCaseBuilder {
            testcase: TestCase::skipped(name),
        }
    }

    /// Build and return a [`TestCase`](struct.TestCase.html) object based on the data stored in this TestCaseBuilder object.
    pub fn build(&self) -> TestCase {
        self.testcase.clone()
    }
}

// Make sure the readme is tested too
#[cfg(doctest)]
doc_comment::doctest!("../README.md");
