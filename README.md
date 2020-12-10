# JUnit Report in Rust

Generate JUnit compatible XML reports in Rust.

## Example

```rust

    extern crate junit_report;

    use junit_report::{Report, TestCase, TestSuite, Duration, TimeZone, Utc};
    use std::fs::File;

    // Create a successful test case
    let test_success = TestCase::success("good test", Duration::seconds(15)).set_classname("MyClass");

    // Create a test case that encountered an unexpected error condition
    let test_error = TestCase::error(
        "error test",
        Duration::seconds(5),
        "git error",
        "unable to fetch",
    );

    // Create a test case that failed because of a test failure
    let test_failure = TestCase::failure(
        "failure test",
        Duration::seconds(10),
        "assert_eq",
        "not equal",
    );

    // Next we create a test suite named "ts1" with not test cases associated
    let ts1 = TestSuite::new("ts1");

    // Then we create a second test suite called "ts2" and set an explicit time stamp
    // then we add all the test cases from above
    let timestamp = Utc.ymd(1970, 1, 1).and_hms(0, 1, 1);
    let ts2 = TestSuite::new("ts2").set_timestamp(timestamp)
        .add_testcase(test_success)
        .add_testcase(test_error)
        .add_testcase(test_failure);

    // Last we create a report and add all test suites to it
    let r = Report::new()
        .add_testsuite(ts1)
        .add_testsuite(ts2);

    // The report can than be written in XML format to any writer
    let mut file = File::create("my-junit.xml").unwrap();
    r.write_xml(&mut file).unwrap();
```
