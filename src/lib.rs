/*
 * Copyright (c) 2018 Pascal Bach
 *
 * SPDX-License-Identifier:     MIT
 */

/// Create JUnit compatible XML reports.
///
/// ## Example
///
/// ```rust
///
///     use junit_report::{Report, TestCase, TestSuite, Duration, TimeZone, Utc};
///     use std::str;
///
///
///     let timestamp = Utc.ymd(1970, 1, 1).and_hms(0, 1, 1);
///
///     let mut r = Report::new();
///     let mut ts1 = TestSuite::new("ts1");
///     ts1.set_timestamp(timestamp);
///     let mut ts2 = TestSuite::new("ts2");
///     ts2.set_timestamp(timestamp);
///
///     let test_success = TestCase::success("good test", Duration::seconds(15));
///     let test_error = TestCase::error(
///         "error test",
///         Duration::seconds(5),
///         "git error",
///         "unable to fetch",
///     );
///     let test_failure = TestCase::failure(
///         "failure test",
///         Duration::seconds(10),
///         "assert_eq",
///         "not equal",
///     );
///
///     ts2.add_testcase(test_success);
///     ts2.add_testcase(test_error);
///     ts2.add_testcase(test_failure);
///
///     r.add_testsuite(ts1);
///     r.add_testsuite(ts2);
///
///     let mut out: Vec<u8> = Vec::new();
///
///     r.write_xml(&mut out).unwrap();
/// ```

use std::io::Write;

use derive_getters::Getters;
use xml::writer::{self, EmitterConfig, XmlEvent};

pub use chrono::{DateTime, Duration, TimeZone, Utc};

fn decimal_seconds(d: &Duration) -> f64 {
    if let Some(n) = d.num_nanoseconds() {
        n as f64 / 1_000_000_000.0
    } else if let Some(n) = d.num_microseconds() {
        n as f64 / 1_000_000.0
    } else {
        d.num_milliseconds() as f64 / 1_000.0
    }
}

/// Root element of a JUnit report
#[derive(Default, Debug, Clone, Getters)]
pub struct Report {
    testsuites: Vec<TestSuite>,
}

impl Report {
    /// Create a new empty Report
    pub fn new() -> Report {
        Report {
            testsuites: Vec::new(),
        }
    }

    /// Add a [`TestSuite`](../struct.TestSuite.html) to this report.
    ///
    /// The function takes ownership of the supplied [`TestSuite`](../struct.TestSuite.html).
    pub fn add_testsuite(&mut self, testsuite: TestSuite) {
        self.testsuites.push(testsuite);
    }

    /// Add multiple[`TestSuite`s](../struct.TestSuite.html) from an iterator.
    pub fn add_testsuites(&mut self, testsuites: impl IntoIterator<Item = TestSuite>) {
        self.testsuites.extend(testsuites);
    }

    //TODO: Use custom error to not expose xml-rs, maybe via failure
    /// Write the XML version of the Report to the given `Writer`.
    pub fn write_xml<W: Write>(&self, sink: W) -> writer::Result<()> {
        let mut ew = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(sink);
        ew.write(XmlEvent::start_element("testsuites"))?;

        for (id, ts) in self.testsuites.iter().enumerate() {
            ew.write(
                XmlEvent::start_element("testsuite")
                    .attr("id", &format!("{}", id))
                    .attr("name", &ts.name)
                    .attr("package", &ts.package)
                    .attr("tests", &format!("{}", &ts.tests()))
                    .attr("errors", &format!("{}", &ts.errors()))
                    .attr("failures", &format!("{}", &ts.failures()))
                    .attr("hostname", &ts.hostname)
                    .attr("timestamp", &ts.timestamp.to_rfc3339())
                    .attr("time", &format!("{}", decimal_seconds(&ts.time()))),
            )?;

            //TODO: support properties
            //ew.write(XmlEvent::start_element("properties"))?;
            //ew.write(XmlEvent::end_element())?;

            for tc in &ts.testcases {
                ew.write(
                    XmlEvent::start_element("testcase")
                        .attr("name", &tc.name)
                        .attr("time", &format!("{}", decimal_seconds(&tc.time))),
                )?;

                match tc.result {
                    TestResult::Success => {}
                    TestResult::Error {
                        ref type_,
                        ref message,
                    } => {
                        ew.write(
                            XmlEvent::start_element("error")
                                .attr("type", &type_)
                                .attr("message", &message),
                        )?;
                        ew.write(XmlEvent::end_element())?;
                    }
                    TestResult::Failure {
                        ref type_,
                        ref message,
                    } => {
                        ew.write(
                            XmlEvent::start_element("failure")
                                .attr("type", &type_)
                                .attr("message", &message),
                        )?;
                        ew.write(XmlEvent::end_element())?;
                    }
                };

                ew.write(XmlEvent::end_element())?;
            }

            //TODO: support system-out
            ew.write(XmlEvent::start_element("system-out"))?;
            ew.write(XmlEvent::end_element())?;

            //TODO: support system-err
            ew.write(XmlEvent::start_element("system-err"))?;
            ew.write(XmlEvent::end_element())?;

            ew.write(XmlEvent::end_element())?;
        }

        ew.write(XmlEvent::end_element())?;

        Ok(())
    }
}

/// A `TestSuite` groups together several [`TestCase`s](../struct.TestCase.html).
#[derive(Debug, Clone, Getters)]
pub struct TestSuite {
    name: String,
    package: String,
    timestamp: DateTime<Utc>,
    hostname: String,
    testcases: Vec<TestCase>,
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

    fn tests(&self) -> usize {
        self.testcases.len()
    }

    fn errors(&self) -> usize {
        self.testcases.iter().filter(|x| x.is_error()).count()
    }

    fn failures(&self) -> usize {
        self.testcases.iter().filter(|x| x.is_failure()).count()
    }

    fn time(&self) -> Duration {
        self.testcases
            .iter()
            .fold(Duration::zero(), |sum, d| sum + d.time)
    }
}

/// One single test case
#[derive(Debug, Clone, Getters)]
pub struct TestCase {
    name: String,
    time: Duration,
    result: TestResult, //TODO: support classname
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
    pub fn success(name: &str, time: Duration) -> TestCase {
        TestCase {
            name: name.into(),
            time,
            result: TestResult::Success,
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
    pub fn error(name: &str, time: Duration, type_: &str, message: &str) -> TestCase {
        TestCase {
            name: name.into(),
            time,
            result: TestResult::Error {
                type_: type_.into(),
                message: message.into(),
            },
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
    pub fn failure(name: &str, time: Duration, type_: &str, message: &str) -> TestCase {
        TestCase {
            name: name.into(),
            time,
            result: TestResult::Failure {
                type_: type_.into(),
                message: message.into(),
            },
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

#[cfg(test)]
mod tests {
    pub fn normalize(out: Vec<u8>) -> String {
        String::from_utf8(out).unwrap().replace("\r\n","\n")
    }

    #[test]
    fn empty_testsuites() {
        use crate::Report;

        let r = Report::new();

        let mut out: Vec<u8> = Vec::new();

        r.write_xml(&mut out).unwrap();

        assert_eq!(
            normalize(out),
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<testsuites />"
        );
    }

    #[test]
    fn add_empty_testsuite_single() {
        use crate::{Report, TestSuite, TimeZone, Utc};

        let timestamp = Utc.ymd(1970, 1, 1).and_hms(0, 1, 1);

        let mut r = Report::new();
        let mut ts1 = TestSuite::new("ts1");
        ts1.set_timestamp(timestamp);
        let mut ts2 = TestSuite::new("ts2");
        ts2.set_timestamp(timestamp);

        r.add_testsuite(ts1);
        r.add_testsuite(ts2);

        let mut out: Vec<u8> = Vec::new();

        r.write_xml(&mut out).unwrap();

        assert_eq!(
            normalize(out),
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<testsuites>\n  <testsuite id=\"0\" name=\"ts1\" package=\"testsuite/ts1\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T00:01:01+00:00\" time=\"0\">\n    <system-out />\n    <system-err />\n  </testsuite>\n  <testsuite id=\"1\" name=\"ts2\" package=\"testsuite/ts2\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T00:01:01+00:00\" time=\"0\">\n    <system-out />\n    <system-err />\n  </testsuite>\n</testsuites>"
        );
    }

    #[test]
    fn add_empty_testsuite_batch() {
        use crate::{Report, TestSuite, TimeZone, Utc};

        let timestamp = Utc.ymd(1970, 1, 1).and_hms(0, 1, 1);

        let mut r = Report::new();
        let mut ts1 = TestSuite::new("ts1");
        ts1.set_timestamp(timestamp);
        let mut ts2 = TestSuite::new("ts2");
        ts2.set_timestamp(timestamp);

        let v = vec![ts1, ts2];

        r.add_testsuites(v);

        let mut out: Vec<u8> = Vec::new();

        r.write_xml(&mut out).unwrap();

        assert_eq!(
            normalize(out),
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<testsuites>\n  <testsuite id=\"0\" name=\"ts1\" package=\"testsuite/ts1\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T00:01:01+00:00\" time=\"0\">\n    <system-out />\n    <system-err />\n  </testsuite>\n  <testsuite id=\"1\" name=\"ts2\" package=\"testsuite/ts2\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T00:01:01+00:00\" time=\"0\">\n    <system-out />\n    <system-err />\n  </testsuite>\n</testsuites>"
        );
    }

    #[test]
    fn count_tests() {
        use crate::{Duration, TestCase, TestSuite};

        let mut ts = TestSuite::new("ts");

        let tc1 = TestCase::success("mysuccess", Duration::milliseconds(6001));
        let tc2 = TestCase::error(
            "myerror",
            Duration::seconds(6),
            "Some Error",
            "An Error happened",
        );
        let tc3 = TestCase::failure(
            "myerror",
            Duration::seconds(6),
            "Some failure",
            "A Failure happened",
        );

        assert_eq!(0, ts.tests());
        assert_eq!(0, ts.errors());
        assert_eq!(0, ts.failures());

        ts.add_testcase(tc1);

        assert_eq!(1, ts.tests());
        assert_eq!(0, ts.errors());
        assert_eq!(0, ts.failures());

        ts.add_testcase(tc2);

        assert_eq!(2, ts.tests());
        assert_eq!(1, ts.errors());
        assert_eq!(0, ts.failures());

        ts.add_testcase(tc3);

        assert_eq!(3, ts.tests());
        assert_eq!(1, ts.errors());
        assert_eq!(1, ts.failures());
    }

    #[test]
    fn testcases() {
        use crate::{Duration, Report, TestCase, TestSuite, TimeZone, Utc};

        let timestamp = Utc.ymd(1970, 1, 1).and_hms(0, 1, 1);

        let mut r = Report::new();
        let mut ts1 = TestSuite::new("ts1");
        ts1.set_timestamp(timestamp);
        let mut ts2 = TestSuite::new("ts2");
        ts2.set_timestamp(timestamp);

        let test_success = TestCase::success("good test", Duration::milliseconds(15001));
        let test_error = TestCase::error(
            "error test",
            Duration::seconds(5),
            "git error",
            "unable to fetch",
        );
        let test_failure = TestCase::failure(
            "failure test",
            Duration::seconds(10),
            "assert_eq",
            "not equal",
        );

        ts2.add_testcase(test_success);
        ts2.add_testcase(test_error);
        ts2.add_testcase(test_failure);

        r.add_testsuite(ts1);
        r.add_testsuite(ts2);

        let mut out: Vec<u8> = Vec::new();

        r.write_xml(&mut out).unwrap();

        assert_eq!(
            normalize(out),
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<testsuites>\n  <testsuite id=\"0\" name=\"ts1\" package=\"testsuite/ts1\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T00:01:01+00:00\" time=\"0\">\n    <system-out />\n    <system-err />\n  </testsuite>\n  <testsuite id=\"1\" name=\"ts2\" package=\"testsuite/ts2\" tests=\"3\" errors=\"1\" failures=\"1\" hostname=\"localhost\" timestamp=\"1970-01-01T00:01:01+00:00\" time=\"30.001\">\n    <testcase name=\"good test\" time=\"15.001\" />\n    <testcase name=\"error test\" time=\"5\">\n      <error type=\"git error\" message=\"unable to fetch\" />\n    </testcase>\n    <testcase name=\"failure test\" time=\"10\">\n      <failure type=\"assert_eq\" message=\"not equal\" />\n    </testcase>\n    <system-out />\n    <system-err />\n  </testsuite>\n</testsuites>"
        );
    }
}
