/*
 * Copyright (c) 2018 Pascal Bach
 * Copyright (c) 2021 Siemens Mobility GmbH
 *
 * SPDX-License-Identifier:     MIT
 */

use std::io::Write;

use crate::collections::{TestResult, TestSuite};
use derive_getters::Getters;
use time::format_description::well_known::Rfc3339;
use xml::writer::{EmitterConfig, XmlEvent};

use thiserror::Error;

#[derive(Error, Debug)]
/// Errors that can occur when creating a `Report`
pub enum ReportError {
    #[error("unable to parse the input")]
    Io(#[from] std::io::Error),
    #[error("unable to write report")]
    Write(#[from] xml::writer::Error),
}

/// Root element of a JUnit report
#[derive(Default, Debug, Clone, Getters)]
pub struct Report {
    testsuites: Vec<TestSuite>,
}

impl Report {
    /// Create a new empty Report
    pub fn new() -> Report {
        Report {
            testsuites: Vec::new(),
        }
    }

    /// Add a [`TestSuite`](struct.TestSuite.html) to this report.
    ///
    /// The function takes ownership of the supplied [`TestSuite`](struct.TestSuite.html).
    pub fn add_testsuite(&mut self, testsuite: TestSuite) {
        self.testsuites.push(testsuite);
    }

    /// Add multiple[`TestSuite`s](struct.TestSuite.html) from an iterator.
    pub fn add_testsuites(&mut self, testsuites: impl IntoIterator<Item = TestSuite>) {
        self.testsuites.extend(testsuites);
    }

    /// Write the XML version of the Report to the given `Writer`.
    pub fn write_xml<W: Write>(&self, sink: W) -> Result<(), ReportError> {
        let mut ew = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(sink);
        ew.write(XmlEvent::start_element("testsuites"))?;

        for (id, ts) in self.testsuites.iter().enumerate() {
            ew.write(
                XmlEvent::start_element("testsuite")
                    .attr("id", &format!("{}", id))
                    .attr("name", &ts.name)
                    .attr("package", &ts.package)
                    .attr("tests", &format!("{}", &ts.tests()))
                    .attr("errors", &format!("{}", &ts.errors()))
                    .attr("failures", &format!("{}", &ts.failures()))
                    .attr("hostname", &ts.hostname)
                    .attr(
                        "timestamp",
                        &ts.timestamp.format(&Rfc3339).expect("failed to format"),
                    )
                    .attr("time", &format!("{}", ts.time().as_seconds_f64())),
            )?;

            //TODO: support properties
            //ew.write(XmlEvent::start_element("properties"))?;
            //ew.write(XmlEvent::end_element())?;

            for tc in &ts.testcases {
                let time = format!("{}", tc.time.as_seconds_f64());
                let mut testcase_element = XmlEvent::start_element("testcase")
                    .attr("name", &tc.name)
                    .attr("time", &time);

                if let Some(classname) = &tc.classname {
                    testcase_element = testcase_element.attr("classname", classname);
                }

                if let Some(filepath) = &tc.filepath {
                    testcase_element = testcase_element.attr("file", filepath);
                }

                ew.write(testcase_element)?;

                match tc.result {
                    TestResult::Success => {
                        if let Some(system_out) = &tc.system_out {
                            ew.write(XmlEvent::start_element("system-out"))?;
                            ew.write(XmlEvent::CData(system_out.as_str()))?;
                            ew.write(XmlEvent::end_element())?;
                        }

                        if let Some(system_err) = &tc.system_err {
                            ew.write(XmlEvent::start_element("system-err"))?;
                            ew.write(XmlEvent::CData(system_err.as_str()))?;
                            ew.write(XmlEvent::end_element())?;
                        }
                    }
                    TestResult::Error {
                        ref type_,
                        ref message,
                    } => {
                        ew.write(
                            XmlEvent::start_element("error")
                                .attr("type", type_)
                                .attr("message", message),
                        )?;
                        if let Some(stdout) = &tc.system_out {
                            let data = strip_ansi_escapes::strip(stdout.as_str())?;
                            ew.write(XmlEvent::CData(&String::from_utf8_lossy(&data)))?;
                        }
                        if let Some(stderr) = &tc.system_err {
                            let data = strip_ansi_escapes::strip(stderr.as_str())?;
                            ew.write(XmlEvent::CData(&String::from_utf8_lossy(&data)))?;
                        }
                        ew.write(XmlEvent::end_element())?;
                    }
                    TestResult::Failure {
                        ref type_,
                        ref message,
                    } => {
                        ew.write(
                            XmlEvent::start_element("failure")
                                .attr("type", type_)
                                .attr("message", message),
                        )?;
                        if let Some(stdout) = &tc.system_out {
                            let data = strip_ansi_escapes::strip(stdout.as_str())?;
                            ew.write(XmlEvent::CData(&String::from_utf8_lossy(&data)))?;
                        }
                        if let Some(stderr) = &tc.system_err {
                            let data = strip_ansi_escapes::strip(stderr.as_str())?;
                            ew.write(XmlEvent::CData(&String::from_utf8_lossy(&data)))?;
                        }
                        ew.write(XmlEvent::end_element())?;
                    }
                    TestResult::Skipped => {
                        ew.write(XmlEvent::start_element("skipped"))?;
                        ew.write(XmlEvent::end_element())?;
                    }
                };

                ew.write(XmlEvent::end_element())?;
            }

            if let Some(system_out) = &ts.system_out {
                ew.write(XmlEvent::start_element("system-out"))?;
                ew.write(XmlEvent::CData(system_out.as_str()))?;
                ew.write(XmlEvent::end_element())?;
            }

            if let Some(system_err) = &ts.system_err {
                ew.write(XmlEvent::start_element("system-err"))?;
                ew.write(XmlEvent::CData(system_err.as_str()))?;
                ew.write(XmlEvent::end_element())?;
            }

            ew.write(XmlEvent::end_element())?;
        }

        ew.write(XmlEvent::end_element())?;

        Ok(())
    }
}

/// Builder for JUnit [`Report`](struct.Report.html) objects
#[derive(Default, Debug, Clone, Getters)]
pub struct ReportBuilder {
    report: Report,
}

impl ReportBuilder {
    /// Create a new empty ReportBuilder
    pub fn new() -> ReportBuilder {
        ReportBuilder {
            report: Report::new(),
        }
    }

    /// Add a [`TestSuite`](struct.TestSuite.html) to this report builder.
    ///
    /// The function takes ownership of the supplied [`TestSuite`](struct.TestSuite.html).
    pub fn add_testsuite(&mut self, testsuite: TestSuite) -> &mut Self {
        self.report.testsuites.push(testsuite);
        self
    }

    /// Add multiple[`TestSuite`s](struct.TestSuite.html) from an iterator.
    pub fn add_testsuites(&mut self, testsuites: impl IntoIterator<Item = TestSuite>) -> &mut Self {
        self.report.testsuites.extend(testsuites);
        self
    }

    /// Build and return a [`Report`](struct.Report.html) object based on the data stored in this ReportBuilder object.
    pub fn build(&self) -> Report {
        self.report.clone()
    }
}
