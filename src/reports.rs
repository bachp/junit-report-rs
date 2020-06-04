use std::io::Write;

use crate::collections::{TestResult, TestSuite};
use derive_getters::Getters;
use xml::writer::{EmitterConfig, XmlEvent};

pub use chrono::{DateTime, Duration, TimeZone, Utc};

use thiserror::Error;

#[derive(Error, Debug)]
/// Errors that can occur when creating a `Report`
pub enum ReportError {
    #[error("unable to write report")]
    Write(#[from] xml::writer::Error),
}

fn decimal_seconds(d: &Duration) -> f64 {
    if let Some(n) = d.num_nanoseconds() {
        n as f64 / 1_000_000_000.0
    } else if let Some(n) = d.num_microseconds() {
        n as f64 / 1_000_000.0
    } else {
        d.num_milliseconds() as f64 / 1_000.0
    }
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

    /// Add a [`TestSuite`](../struct.TestSuite.html) to this report.
    ///
    /// The function takes ownership of the supplied [`TestSuite`](../struct.TestSuite.html).
    pub fn add_testsuite(mut self, testsuite: TestSuite) -> Self {
        self.testsuites.push(testsuite);
        self
    }

    /// Add multiple[`TestSuite`s](../struct.TestSuite.html) from an iterator.
    pub fn add_testsuites(mut self, testsuites: impl IntoIterator<Item = TestSuite>) -> Self {
        self.testsuites.extend(testsuites);
        self
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
                    .attr("timestamp", &ts.timestamp.to_rfc3339())
                    .attr("time", &format!("{}", decimal_seconds(&ts.time()))),
            )?;

            //TODO: support properties
            //ew.write(XmlEvent::start_element("properties"))?;
            //ew.write(XmlEvent::end_element())?;

            for tc in &ts.testcases {
                if let Some(classname) = &tc.classname {
                    ew.write(
                        XmlEvent::start_element("testcase")
                            .attr("name", &tc.name)
                            .attr("classname", classname)
                            .attr("time", &format!("{}", decimal_seconds(&tc.time))),
                    )?;
                } else {
                    ew.write(
                        XmlEvent::start_element("testcase")
                            .attr("name", &tc.name)
                            .attr("time", &format!("{}", decimal_seconds(&tc.time))),
                    )?;
                }

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

                match tc.result {
                    TestResult::Success => {}
                    TestResult::Error {
                        ref type_,
                        ref message,
                    } => {
                        ew.write(
                            XmlEvent::start_element("error")
                                .attr("type", &type_)
                                .attr("message", &message),
                        )?;
                        ew.write(XmlEvent::end_element())?;
                    }
                    TestResult::Failure {
                        ref type_,
                        ref message,
                    } => {
                        ew.write(
                            XmlEvent::start_element("failure")
                                .attr("type", &type_)
                                .attr("message", &message),
                        )?;
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
