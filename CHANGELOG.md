# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Remove strip-ansi-escapes dependencies
- Only enable required features for time

## [0.8.4] - 2023-12-07

- Update dependencies
- `system-err` and `system-out` are now independent of the test case result
- Add `set_trace` function to add detailed trace to error or failure cases.

## [0.8.3] - 2023-10-23

- Update dependencies

## [0.8.2] - 2022-12-16

- Re-export `quick_xml::Error` as `junit_report::Error`

## [0.8.1] - 2022-09-10

- Remove unsecure dev dependency from `failure`

## [0.8.0] - 2022-09-09

- Bump Rust edition to 2021

### BREAKING CHANGES
- Switch from `xml-rs` to `quick-xml` due to maintenance status
  - Change `Err` type of `Report::write_xml()`
  - Remove indentations and newlines from the generated `Report`

## [0.7.1] - 2022-04-27

- Added support for an optional `file` attribute in test cases

## [0.7.0] - 2021-11-06

### BREAKING CHANGES
- Switch from `chrono` to `time`
- Switch timestamp formatting (still compliant with both `RFC3339` and `ISO8601`)

## [0.6.0] - 2021-07-20

- Saparate builder types

### BREAKING CHANGES
- Seprate types for the data types and the builders. This restores the old data based API from 0.3.0 and moves
the builder API as introduced in 0.4.0 to their own *Builder types.
- If you are migrating from 0.3.0 there should be no big changes required.
- If you migrate from 0.4.0 or 0.5.0 you need the following renames:
  Report -> ReportBuilder
  TestSuite -> TestSuiteBuilder
  TestCase -> TestCaseBuilder

## [0.5.0] - 2021-06-15

### Added
- Support for skipped or ignored testcases
### BREAKING CHANGES
- Adding support for skipped and ignored testcases extends the `TestResult` struct by one more variant.

## [0.4.2] - 2021-05-28

### Fixed

- Make Error Type public

## [0.4.1] - 2021-03-02

### Fixed

- Output format compatible with GitLab and Jenkins.

## [0.4.0] - 2020-06-04

### Added
- `system_out` and `system_err` fields added

### BREAKING CHANGES
- Revamp the API to use the builder pattern. This makes the API more future proof and hopefully avoids breaking changes in the future when more optional fields are added.
- Change error type to no longer expose the internals of the XML processing.

## [0.3.0] - 2020-05-12

### Added
- `classname` attribute is now supported

## [0.2.1] - 2020-04-14
### Changed
- Make sure all examples in the readme are run
- Update dependencies

## [0.2.0] - 2019-08-19
### Fixed
- Testsuite id is now properly set even when using `add_testsuites`
- Unittests now work in Windows too

### Changed
- Crate now uses the Rust 2018 edition
- The batch methods (`add_testsuites`, `add_testcases`) now accept any iterators, not just `Vec`
- Durations are now decimals as per spec

## [0.1.2] - 2018-11-22
### Changed
- Change order to `system-out` and `system-err` to conform to new schema
- Don't add an empty optional properties tag

## [0.1.1] - 2018-09-22
### Added
- Add functions to add testcases and testsuites from a Vec

## [0.1.0] - 2018-09-21
### Added
- Initial Release
