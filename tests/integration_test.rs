/*
 * Copyright (c) 2018 Pascal Bach
 *
 * SPDX-License-Identifier:     MIT
 */

extern crate chrono;
extern crate junit_report;

#[test]
fn reference_report() {
    use chrono::{Duration, TimeZone, Utc};
    use junit_report::{Report, TestCase, TestSuite};
    use std::fs::File;
    use std::io::Read;

    let timestamp = Utc.ymd(2018, 4, 21).and_hms(12, 02, 0);

    let mut r = Report::new();
    let mut ts1 = TestSuite::new("ts1");
    ts1.set_timestamp(timestamp);

    let test_success = TestCase::success("test1", Duration::seconds(15));
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

    ts1.add_testcase(test_success);
    ts1.add_testcase(test_failure);
    ts1.add_testcase(test_error);

    r.add_testsuite(ts1);

    let mut out: Vec<u8> = Vec::new();

    r.write_xml(&mut out).unwrap();

    let report = String::from_utf8(out).unwrap().replace("\r\n","\n");

    let mut reference = String::new();

    let mut f = File::open("tests/reference.xml").unwrap();
    f.read_to_string(&mut reference).unwrap();
    let reference = reference.replace("\r\n","\n");

    assert_eq!(report, reference);
}

#[macro_use]
extern crate commandspec;

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
    use chrono::{Duration, TimeZone, Utc};
    use junit_report::{Report, TestCase, TestSuite};
    use std::fs::File;

    let timestamp = Utc.ymd(2018, 4, 21).and_hms(12, 02, 0);

    let mut r = Report::new();
    let mut ts1 = TestSuite::new("Some Testsuite");
    ts1.set_timestamp(timestamp);

    let test_success = TestCase::success("MyTest3", Duration::seconds(15));
    let test_error = TestCase::error(
        "Blabla",
        Duration::seconds(5),
        "git error",
        "Could not clone",
    );
    let test_failure = TestCase::failure("Burk", Duration::seconds(10), "asdfasf", "asdfajfhk");

    ts1.add_testcase(test_success);
    ts1.add_testcase(test_failure);
    ts1.add_testcase(test_error);

    r.add_testsuite(ts1);

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
