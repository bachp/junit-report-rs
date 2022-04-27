<!--
Copyright (c) 2018 Pascal Bach
Copyright (c) 2021 Siemens Mobility GmbH

SPDX-License-Identifier:     MIT
-->

# JUnit Report in Rust

Generate JUnit compatible XML reports in Rust.

## Example

```rust

    use junit_report::{datetime, Duration, ReportBuilder, TestCase, TestCaseBuilder, TestSuite, TestSuiteBuilder};
    use std::fs::File;

    // Create a successful test case
    let test_success = TestCaseBuilder::success("good test", Duration::seconds(15))
        .set_classname("MyClass")
        .set_filepath("MyFilePath")
        .build();

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
    let timestamp = datetime!(1970-01-01 00:01:01 UTC);
    let ts2 = TestSuiteBuilder::new("ts2")
        .set_timestamp(timestamp)
        .add_testcase(test_success)
        .add_testcase(test_error)
        .add_testcase(test_failure)
        .build();

    // Last we create a report and add all test suites to it
    let r = ReportBuilder::new()
        .add_testsuite(ts1)
        .add_testsuite(ts2)
        .build();

    // The report can than be written in XML format to any writer
    let mut file = File::create("my-junit.xml").unwrap();
    r.write_xml(&mut file).unwrap();
```
