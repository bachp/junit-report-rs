/*
 * Copyright (c) 2018 Pascal Bach
 *
 * SPDX-License-Identifier:     MIT
 */

/// Create JUnit compatible XML reports.
extern crate chrono;
extern crate xml;

use std::io::Write;

use xml::writer::{self, EmitterConfig, XmlEvent};

pub use chrono::DateTime;
pub use chrono::Duration;
use chrono::Utc;

/// Root element of a JUnit report
#[derive(Default)]
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

    /// Add a `TestSuite` to this report.
    ///
    /// The function takes ownership of the supplied TestSuite.
    pub fn add_testsuite(&mut self, mut testsuite: TestSuite) {
        testsuite.id = self.testsuites.len();
        self.testsuites.push(testsuite);
    }

    //TODO: Use custom error to not expose xml-rs, maybe via failure
    /// Write the XML version of the Report to the given Writer
    pub fn write_xml<W: Write>(self, sink: W) -> writer::Result<()> {
        let mut ew = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(sink);
        ew.write(XmlEvent::start_element("testsuites"))?;

        for ts in &self.testsuites {
            ew.write(
                XmlEvent::start_element("testsuite")
                    .attr("id", &format!("{}", &ts.id))
                    .attr("name", &ts.name)
                    .attr("package", &ts.package)
                    .attr("tests", &format!("{}", &ts.tests()))
                    .attr("errors", &format!("{}", &ts.errors()))
                    .attr("failures", &format!("{}", &ts.failures()))
                    .attr("hostname", &ts.hostname)
                    .attr("timestamp", &ts.timestamp.to_rfc3339())
                    .attr("time", &format!("{}", &ts.time().num_seconds())),
            )?;

            //TODO: support properties
            ew.write(XmlEvent::start_element("properties"))?;
            ew.write(XmlEvent::end_element())?;

            //TODO: support system-out
            ew.write(XmlEvent::start_element("system-out"))?;
            ew.write(XmlEvent::end_element())?;

            //TODO: support system-err
            ew.write(XmlEvent::start_element("system-err"))?;
            ew.write(XmlEvent::end_element())?;

            for tc in &ts.testcases {
                ew.write(
                    XmlEvent::start_element("testcase")
                        .attr("name", &tc.name)
                        .attr("time", &format!("{}", &tc.time.num_seconds())),
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

            ew.write(XmlEvent::end_element())?;
        }

        ew.write(XmlEvent::end_element())?;

        Ok(())
    }
}

/// A `TestSuite` groups together several `TestCase`s
pub struct TestSuite {
    name: String,
    id: usize,
    package: String,
    timestamp: DateTime<Utc>,
    hostname: String,
    testcases: Vec<TestCase>,
}

impl TestSuite {
    pub fn new(name: &str) -> TestSuite {
        TestSuite {
            id: 0,
            hostname: "localhost".into(),
            package: format!("testsuite/{}", &name),
            name: name.into(),
            timestamp: Utc::now(),
            testcases: Vec::new(),
        }
    }

    pub fn add_testcase(&mut self, testcase: TestCase) {
        self.testcases.push(testcase);
    }

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
pub struct TestCase {
    name: String,
    time: Duration,
    result: TestResult, //TODO: support classname
}

/// Result of a test case
enum TestResult {
    Success,
    Error { type_: String, message: String },
    Failure { type_: String, message: String },
}

impl TestCase {
    /// Creates a new successful test case
    pub fn success(name: &str, time: Duration) -> TestCase {
        TestCase {
            name: name.into(),
            time,
            result: TestResult::Success,
        }
    }

    /// Check if a test case is successful
    pub fn is_success(&self) -> bool {
        match self.result {
            TestResult::Success => true,
            _ => false,
        }
    }

    /// Creates a new erroneous test case
    ///
    /// An erroneous test case is one that encountered an unexpected error condition.
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

    /// Check if a test case is erroneous
    pub fn is_error(&self) -> bool {
        match self.result {
            TestResult::Error { .. } => true,
            _ => false,
        }
    }

    /// Creates a new failed test case
    ///
    /// A failed test case is one where an explicit asseriton failed
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

    /// Check if a test case failed
    pub fn is_failure(&self) -> bool {
        match self.result {
            TestResult::Failure { .. } => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty_testsuites() {
        use std::str;
        use Report;

        let r = Report::new();

        let mut out: Vec<u8> = Vec::new();

        r.write_xml(&mut out).unwrap();

        assert_eq!(
            str::from_utf8(&out).unwrap(),
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<testsuites />"
        );
    }

    #[test]
    fn empty_testsuite() {
        use chrono::{TimeZone, Utc};
        use std::str;
        use {Report, TestSuite};

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
            str::from_utf8(&out).unwrap(),
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<testsuites>\n  <testsuite id=\"0\" name=\"ts1\" package=\"testsuite/ts1\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T00:01:01+00:00\" time=\"0\">\n    <properties />\n    <system-out />\n    <system-err />\n  </testsuite>\n  <testsuite id=\"1\" name=\"ts2\" package=\"testsuite/ts2\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T00:01:01+00:00\" time=\"0\">\n    <properties />\n    <system-out />\n    <system-err />\n  </testsuite>\n</testsuites>"
        );
    }

    #[test]
    fn count_tests() {
        use {Duration, TestCase, TestSuite};

        let mut ts = TestSuite::new("ts");

        let tc1 = TestCase::success("mysuccess", Duration::seconds(6));
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
        use chrono::{Duration, TimeZone, Utc};
        use std::str;
        use {Report, TestCase, TestSuite};

        let timestamp = Utc.ymd(1970, 1, 1).and_hms(0, 1, 1);

        let mut r = Report::new();
        let mut ts1 = TestSuite::new("ts1");
        ts1.set_timestamp(timestamp);
        let mut ts2 = TestSuite::new("ts2");
        ts2.set_timestamp(timestamp);

        let test_success = TestCase::success("good test", Duration::seconds(15));
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
            str::from_utf8(&out).unwrap(),
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<testsuites>\n  <testsuite id=\"0\" name=\"ts1\" package=\"testsuite/ts1\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T00:01:01+00:00\" time=\"0\">\n    <properties />\n    <system-out />\n    <system-err />\n  </testsuite>\n  <testsuite id=\"1\" name=\"ts2\" package=\"testsuite/ts2\" tests=\"3\" errors=\"1\" failures=\"1\" hostname=\"localhost\" timestamp=\"1970-01-01T00:01:01+00:00\" time=\"30\">\n    <properties />\n    <system-out />\n    <system-err />\n    <testcase name=\"good test\" time=\"15\" />\n    <testcase name=\"error test\" time=\"5\">\n      <error type=\"git error\" message=\"unable to fetch\" />\n    </testcase>\n    <testcase name=\"failure test\" time=\"10\">\n      <failure type=\"assert_eq\" message=\"not equal\" />\n    </testcase>\n  </testsuite>\n</testsuites>"
        );
    }
}
