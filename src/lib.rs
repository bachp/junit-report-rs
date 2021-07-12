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
///     use junit_report::{ReportBuilder, TestCase, TestCaseBuilder, TestSuiteBuilder, Duration, TimeZone, Utc};
///
///
///     let timestamp = Utc.ymd(1970, 1, 1).and_hms(0, 1, 1);
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
///     ).set_classname("classname")
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
pub use chrono::{DateTime, Duration, TimeZone, Utc};

mod collections;
mod reports;

pub use crate::collections::{TestCase, TestCaseBuilder, TestSuite, TestSuiteBuilder};
pub use crate::reports::{Report, ReportBuilder, ReportError};

#[cfg(test)]
mod tests {
  pub fn normalize(out: Vec<u8>) -> String {
    String::from_utf8(out).unwrap().replace("\r\n", "\n")
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
    use crate::ReportBuilder;
    use crate::TestSuiteBuilder;
    use crate::{TimeZone, Utc};

    let timestamp = Utc.ymd(1970, 1, 1).and_hms(0, 1, 1);

    let ts1 = TestSuiteBuilder::new("ts1")
      .set_timestamp(timestamp)
      .build();
    let ts2 = TestSuiteBuilder::new("ts2")
      .set_timestamp(timestamp)
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
  <testsuite id=\"0\" name=\"ts1\" package=\"testsuite/ts1\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T00:01:01+00:00\" time=\"0\" />
  <testsuite id=\"1\" name=\"ts2\" package=\"testsuite/ts2\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T00:01:01+00:00\" time=\"0\" />
</testsuites>"
        );
  }

  #[test]
  fn add_empty_testsuite_single_with_sysout() {
    use crate::ReportBuilder;
    use crate::TestSuiteBuilder;
    use crate::{TimeZone, Utc};

    let timestamp = Utc.ymd(1970, 1, 1).and_hms(0, 1, 1);

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
  <testsuite id=\"0\" name=\"ts1\" package=\"testsuite/ts1\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T00:01:01+00:00\" time=\"0\">
    <system-out><![CDATA[Test sysout]]></system-out>
  </testsuite>
</testsuites>"
        );
  }

  #[test]
  fn add_empty_testsuite_single_with_syserror() {
    use crate::ReportBuilder;
    use crate::TestSuiteBuilder;
    use crate::{TimeZone, Utc};

    let timestamp = Utc.ymd(1970, 1, 1).and_hms(0, 1, 1);

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
  <testsuite id=\"0\" name=\"ts1\" package=\"testsuite/ts1\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T00:01:01+00:00\" time=\"0\">
    <system-err><![CDATA[Test syserror]]></system-err>
  </testsuite>
</testsuites>"
        );
  }

  #[test]
  fn add_empty_testsuite_batch() {
    use crate::ReportBuilder;
    use crate::TestSuiteBuilder;
    use crate::{TimeZone, Utc};

    let timestamp = Utc.ymd(1970, 1, 1).and_hms(0, 1, 1);

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
  <testsuite id=\"0\" name=\"ts1\" package=\"testsuite/ts1\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T00:01:01+00:00\" time=\"0\" />
  <testsuite id=\"1\" name=\"ts2\" package=\"testsuite/ts2\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T00:01:01+00:00\" time=\"0\" />
</testsuites>"
        );
  }

  #[test]
  fn count_tests() {
    use crate::Duration;
    use crate::{TestCase, TestSuite};

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
    use crate::{Duration, ReportBuilder, TestCaseBuilder, TestSuiteBuilder, TimeZone, Utc};

    let timestamp = Utc.ymd(1970, 1, 1).and_hms(0, 1, 1);

    let test_success = TestCaseBuilder::success("good test", Duration::milliseconds(15001))
      .set_classname("MyClass")
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
  <testsuite id=\"0\" name=\"ts1\" package=\"testsuite/ts1\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T00:01:01+00:00\" time=\"0\" />
  <testsuite id=\"1\" name=\"ts2\" package=\"testsuite/ts2\" tests=\"3\" errors=\"1\" failures=\"1\" hostname=\"localhost\" timestamp=\"1970-01-01T00:01:01+00:00\" time=\"30.001\">
    <testcase name=\"good test\" classname=\"MyClass\" time=\"15.001\" />
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
    use crate::{Duration, ReportBuilder, TestCaseBuilder, TestSuiteBuilder, TimeZone, Utc};

    let timestamp = Utc.ymd(1970, 1, 1).and_hms(0, 1, 1);

    let test_success = TestCaseBuilder::success("good test", Duration::milliseconds(15001))
      .set_classname("MyClass")
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
  <testsuite id=\"0\" name=\"ts1\" package=\"testsuite/ts1\" tests=\"0\" errors=\"0\" failures=\"0\" hostname=\"localhost\" timestamp=\"1970-01-01T00:01:01+00:00\" time=\"0\" />
  <testsuite id=\"1\" name=\"ts2\" package=\"testsuite/ts2\" tests=\"3\" errors=\"1\" failures=\"1\" hostname=\"localhost\" timestamp=\"1970-01-01T00:01:01+00:00\" time=\"30.001\">
    <testcase name=\"good test\" classname=\"MyClass\" time=\"15.001\">
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
