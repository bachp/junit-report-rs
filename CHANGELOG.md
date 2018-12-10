# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Fixed
- Testsuite id is now properly set even when using `add_testsuites`

### Changed
- Crate now uses the Rust 2018 edition
- The batch methods (`add_testsuites`, `add_testcases`) now accept any iterators, not just `Vec`

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
