# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2020-06-04

## Added
- `system_out` and `system_err` fields added

## BREAKING CHANGES
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
