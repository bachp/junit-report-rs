/*
 * Copyright (c) 2018 Pascal Bach
 * Copyright (c) 2021 Siemens Mobility GmbH
 *
 * SPDX-License-Identifier:     MIT
 */

/// Create JUnit compatible XML reports.
///
/// ## Example
///
/// ```rust
///     use junit_report::{datetime, Duration, ReportBuilder, TestCase, TestCaseBuilder, TestSuiteBuilder};
///
///     let timestamp = datetime!(1970-01-01 01:01 UTC);
///
///     let test_success = TestCase::success("good test", Duration::seconds(15));
///     let test_error = TestCase::error(
///         "error test",
///         Duration::seconds(5),
///         "git error",
///         "unable to fetch",
///     );
///     let test_failure = TestCaseBuilder::failure(
///         "failure test",
///         Duration::seconds(10),
///         "assert_eq",
///         "not equal",
///     ).set_classname("classname").set_filepath("./foo.rs")
///     .build();
///
///     let ts1 = TestSuiteBuilder::new("ts1").set_timestamp(timestamp).build();
///
///     let ts2 = TestSuiteBuilder::new("ts2").set_timestamp(timestamp)
///       .add_testcase(test_success)
///       .add_testcase(test_error)
///       .add_testcase(test_failure)
///       .build();
///
///     let r = ReportBuilder::new()
///       .add_testsuite(ts1)
///       .add_testsuite(ts2)
///       .build();
///
///     let mut out: Vec<u8> = Vec::new();
///
///     r.write_xml(&mut out).unwrap();
/// ```
mod collections;
mod reports;

pub use time::{macros::datetime, Duration, OffsetDateTime};

pub use crate::collections::{TestCase, TestCaseBuilder, TestSuite, TestSuiteBuilder};
pub use crate::reports::{Report, ReportBuilder, ReportError};

#[cfg(test)]
mod tests {
    use crate::{
        datetime, Duration, Report, ReportBuilder, TestCase, TestCaseBuilder, TestSuite,
        TestSuiteBuilder,
    };

    pub fn normalize(out: Vec<u8>) -> String {
        String::from_utf8(out).unwrap().replace("\r\n", "\n")
    }

    #[test]
    fn empty_testsuites() {
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
        let timestamp = datetime!(1970-01-01 01:01 UTC);

        let ts1 = TestSuiteBuilder::new("ts1")
            .set_timestamp(timestamp)
            .build();
        let mut tsb = TestSuiteBuilder::new("ts2");
        tsb.set_timestamp(timestamp);
        let ts2 = tsb.build();

        let r = ReportBuilder::new()
            .add_testsuite(ts1)
            .add_testsuite(ts2)
            .build();

        let mut out: Vec<u8> = Vec::new();

        r.write_xml(&mut out).unwrap();

        assert_eq!(
            normalize(out),
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>
<testsuites>
  <testsuite id=\"0\" name=\"ts1\" package=\"testsuite/ts1\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T01:01:00Z\" time=\"0\" />
  <testsuite id=\"1\" name=\"ts2\" package=\"testsuite/ts2\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T01:01:00Z\" time=\"0\" />
</testsuites>"
        );
    }

    #[test]
    fn add_empty_testsuite_single_with_sysout() {
        let timestamp = datetime!(1970-01-01 01:01 UTC);

        let ts1 = TestSuiteBuilder::new("ts1")
            .set_system_out("Test sysout")
            .set_timestamp(timestamp)
            .build();

        let r = ReportBuilder::new().add_testsuite(ts1).build();

        let mut out: Vec<u8> = Vec::new();

        r.write_xml(&mut out).unwrap();

        assert_eq!(
            normalize(out),
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>
<testsuites>
  <testsuite id=\"0\" name=\"ts1\" package=\"testsuite/ts1\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T01:01:00Z\" time=\"0\">
    <system-out><![CDATA[Test sysout]]></system-out>
  </testsuite>
</testsuites>"
        );
    }

    #[test]
    fn add_empty_testsuite_single_with_syserror() {
        let timestamp = datetime!(1970-01-01 01:01 UTC);

        let ts1 = TestSuiteBuilder::new("ts1")
            .set_system_err("Test syserror")
            .set_timestamp(timestamp)
            .build();

        let r = ReportBuilder::new().add_testsuite(ts1).build();

        let mut out: Vec<u8> = Vec::new();

        r.write_xml(&mut out).unwrap();

        assert_eq!(
            normalize(out),
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>
<testsuites>
  <testsuite id=\"0\" name=\"ts1\" package=\"testsuite/ts1\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T01:01:00Z\" time=\"0\">
    <system-err><![CDATA[Test syserror]]></system-err>
  </testsuite>
</testsuites>"
        );
    }

    #[test]
    fn add_empty_testsuite_batch() {
        let timestamp = datetime!(1970-01-01 01:01 UTC);

        let ts1 = TestSuiteBuilder::new("ts1")
            .set_timestamp(timestamp)
            .build();
        let ts2 = TestSuiteBuilder::new("ts2")
            .set_timestamp(timestamp)
            .build();

        let v = vec![ts1, ts2];

        let r = ReportBuilder::new().add_testsuites(v).build();

        let mut out: Vec<u8> = Vec::new();

        r.write_xml(&mut out).unwrap();

        assert_eq!(
            normalize(out),
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>
<testsuites>
  <testsuite id=\"0\" name=\"ts1\" package=\"testsuite/ts1\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T01:01:00Z\" time=\"0\" />
  <testsuite id=\"1\" name=\"ts2\" package=\"testsuite/ts2\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T01:01:00Z\" time=\"0\" />
</testsuites>"
        );
    }

    #[test]
    fn count_tests() {
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
    fn testcases_no_stdout_stderr() {
        let timestamp = datetime!(1970-01-01 01:01 UTC);

        let test_success = TestCaseBuilder::success("good test", Duration::milliseconds(15001))
            .set_classname("MyClass")
            .set_filepath("./foo.rs")
            .build();
        let test_error = TestCaseBuilder::error(
            "error test",
            Duration::seconds(5),
            "git error",
            "unable to fetch",
        )
        .build();
        let test_failure = TestCaseBuilder::failure(
            "failure test",
            Duration::seconds(10),
            "assert_eq",
            "not equal",
        )
        .build();

        let ts1 = TestSuiteBuilder::new("ts1")
            .set_timestamp(timestamp)
            .build();
        let ts2 = TestSuiteBuilder::new("ts2")
            .set_timestamp(timestamp)
            .add_testcase(test_success)
            .add_testcase(test_error)
            .add_testcase(test_failure)
            .build();

        let r = ReportBuilder::new()
            .add_testsuite(ts1)
            .add_testsuite(ts2)
            .build();

        let mut out: Vec<u8> = Vec::new();

        r.write_xml(&mut out).unwrap();

        assert_eq!(
            normalize(out),
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>
<testsuites>
  <testsuite id=\"0\" name=\"ts1\" package=\"testsuite/ts1\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T01:01:00Z\" time=\"0\" />
  <testsuite id=\"1\" name=\"ts2\" package=\"testsuite/ts2\" tests=\"3\" errors=\"1\" failures=\"1\" hostname=\"localhost\" timestamp=\"1970-01-01T01:01:00Z\" time=\"30.001\">
    <testcase name=\"good test\" time=\"15.001\" classname=\"MyClass\" file=\"./foo.rs\" />
    <testcase name=\"error test\" time=\"5\">
      <error type=\"git error\" message=\"unable to fetch\" />
    </testcase>
    <testcase name=\"failure test\" time=\"10\">
      <failure type=\"assert_eq\" message=\"not equal\" />
    </testcase>
  </testsuite>
</testsuites>"
        );
    }

    #[test]
    fn test_cases_with_sysout_and_syserr() {
        let timestamp = datetime!(1970-01-01 01:01 UTC);

        let test_success = TestCaseBuilder::success("good test", Duration::milliseconds(15001))
            .set_classname("MyClass")
            .set_filepath("./foo.rs")
            .set_system_out("Some sysout message")
            .build();
        let test_error = TestCaseBuilder::error(
            "error test",
            Duration::seconds(5),
            "git error",
            "unable to fetch",
        )
        .set_system_err("Some syserror message")
        .build();
        let test_failure = TestCaseBuilder::failure(
            "failure test",
            Duration::seconds(10),
            "assert_eq",
            "not equal",
        )
        .set_system_out("System out or error message")
        .set_system_err("Another system error message")
        .build();

        let ts1 = TestSuiteBuilder::new("ts1")
            .set_timestamp(timestamp)
            .build();
        let ts2 = TestSuiteBuilder::new("ts2")
            .set_timestamp(timestamp)
            .add_testcase(test_success)
            .add_testcase(test_error)
            .add_testcase(test_failure)
            .build();

        let r = ReportBuilder::new()
            .add_testsuite(ts1)
            .add_testsuite(ts2)
            .build();

        let mut out: Vec<u8> = Vec::new();

        r.write_xml(&mut out).unwrap();

        assert_eq!(
            normalize(out),
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>
<testsuites>
  <testsuite id=\"0\" name=\"ts1\" package=\"testsuite/ts1\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T01:01:00Z\" time=\"0\" />
  <testsuite id=\"1\" name=\"ts2\" package=\"testsuite/ts2\" tests=\"3\" errors=\"1\" failures=\"1\" hostname=\"localhost\" timestamp=\"1970-01-01T01:01:00Z\" time=\"30.001\">
    <testcase name=\"good test\" time=\"15.001\" classname=\"MyClass\" file=\"./foo.rs\">
      <system-out><![CDATA[Some sysout message]]></system-out>
    </testcase>
    <testcase name=\"error test\" time=\"5\">
      <error type=\"git error\" message=\"unable to fetch\"><![CDATA[Some syserror message]]></error>
    </testcase>
    <testcase name=\"failure test\" time=\"10\">
      <failure type=\"assert_eq\" message=\"not equal\"><![CDATA[System out or error message]]><![CDATA[Another system error message]]></failure>
    </testcase>
  </testsuite>
</testsuites>"
        );
    }
}
