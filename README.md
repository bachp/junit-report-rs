# JUnit Report in Rust

Generate JUnit compatible XML reports in Rust.

## Example

```rust

    extern crate junit_report;

    use junit_report::{Report, TestCase, TestSuite, Duration, TimeZone, Utc};
    use std::str;

    let timestamp = Utc.ymd(1970, 1, 1).and_hms(0, 1, 1);

    let test_success = TestCase::success("good test", Duration::seconds(15)).set_classname("MyClass");
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

    let ts1 = TestSuite::new("ts1").set_timestamp(timestamp);

    let ts2 = TestSuite::new("ts2").set_timestamp(timestamp)
        .add_testcase(test_success)
        .add_testcase(test_error)
        .add_testcase(test_failure);

    let r = Report::new()
        .add_testsuite(ts1)
        .add_testsuite(ts2);

    let mut out: Vec<u8> = Vec::new();

    r.write_xml(&mut out).unwrap();
```
