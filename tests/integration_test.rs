/*
 * Copyright (c) 2018 Pascal Bach
 * Copyright (c) 2021 Siemens Mobility GmbH
 *
 * SPDX-License-Identifier:     MIT
 */

use std::fs::{self, File};

use commandspec::sh_command;
use junit_report::{
    datetime, Duration, ReportBuilder, TestCase, TestCaseBuilder, TestSuiteBuilder,
};
use once_cell::sync::Lazy;
use regex::{Regex, RegexBuilder};

static REGEX: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new("\\n|^\\s+")
        .multi_line(true)
        .build()
        .unwrap()
});

#[test]
fn reference_report() {
    let timestamp = datetime!(2018-04-21 12:02 UTC);

    let test_success = TestCaseBuilder::success("test1", Duration::seconds(15))
        .set_classname("MyClass")
        .set_filepath("./foo.rs")
        .build();
    let test_error = TestCase::error(
        "test3",
        Duration::seconds(5),
        "git error",
        "Could not clone",
    );
    let test_failure = TestCase::failure(
        "test2",
        Duration::seconds(10),
        "assert_eq",
        "What was not true",
    );
    let test_ignored = TestCase::skipped("test4");

    let ts1 = TestSuiteBuilder::new("ts1")
        .set_timestamp(timestamp)
        .add_testcase(test_success)
        .add_testcase(test_failure)
        .add_testcase(test_error)
        .add_testcase(test_ignored)
        .build();

    let r = ReportBuilder::new().add_testsuite(ts1).build();

    let mut out: Vec<u8> = Vec::new();

    r.write_xml(&mut out).unwrap();

    let report = String::from_utf8(out).unwrap();

    let reference = fs::read_to_string("tests/reference.xml").unwrap();
    let reference = REGEX.replace_all(reference.as_str(), "");

    assert_eq!(report, reference);
}

#[test]
fn validate_reference_xml_schema() {
    let res = sh_command!(
        r"
        xmllint --schema tests/JUnit.xsd tests/reference.xml --noout
        "
    )
    .unwrap()
    .output()
    .unwrap(); //.expect("reference.xml does not validate against XML Schema")
    print!("{}", String::from_utf8_lossy(&res.stdout));
    eprint!("{}", String::from_utf8_lossy(&res.stderr));
    assert!(res.status.success());
}

#[test]
fn validate_generated_xml_schema() {
    let timestamp = datetime!(2018-04-21 12:02 UTC);

    let test_success = TestCaseBuilder::success("MyTest3", Duration::seconds(15))
        .set_classname("MyClass")
        .build();
    let test_error = TestCase::error(
        "Blabla",
        Duration::seconds(5),
        "git error",
        "Could not clone",
    );
    let test_failure = TestCase::failure("Burk", Duration::seconds(10), "asdfasf", "asdfajfhk");
    let test_skipped = TestCase::skipped("Alpha");

    let ts1 = TestSuiteBuilder::new("Some Testsuite")
        .set_timestamp(timestamp)
        .add_testcase(test_success)
        .add_testcase(test_failure)
        .add_testcase(test_error)
        .add_testcase(test_skipped)
        .build();

    let r = ReportBuilder::new().add_testsuite(ts1).build();

    let mut f = File::create("target/generated.xml").unwrap();

    r.write_xml(&mut f).unwrap();

    let res = sh_command!(
        r"

        xmllint --schema tests/JUnit.xsd target/generated.xml --noout
        "
    )
    .unwrap()
    .output()
    .unwrap(); //.expect("reference.xml does not validate against XML Schema")
    print!("{}", String::from_utf8_lossy(&res.stdout));
    eprint!("{}", String::from_utf8_lossy(&res.stderr));
    assert!(res.status.success());
}
