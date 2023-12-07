/*
 * Copyright (c) 2018 Pascal Bach
 * Copyright (c) 2021 Siemens Mobility GmbH
 *
 * SPDX-License-Identifier:     MIT
 */

use std::io::Write;

use derive_getters::Getters;
use quick_xml::events::BytesDecl;
use quick_xml::{
    events::{BytesCData, Event},
    ElementWriter, Result, Writer,
};
use time::format_description::well_known::Rfc3339;

use crate::{TestCase, TestResult, TestSuite};

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
    pub fn write_xml<W: Write>(&self, sink: W) -> Result<()> {
        let mut writer = Writer::new(sink);

        writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("utf-8"), None)))?;

        writer
            .create_element("testsuites")
            .write_empty_or_inner(
                |_| self.testsuites.is_empty(),
                |w| {
                    w.write_iter(self.testsuites.iter().enumerate(), |w, (id, ts)| {
                        w.create_element("testsuite")
                            .with_attributes([
                                ("id", id.to_string().as_str()),
                                ("name", &ts.name),
                                ("package", &ts.package),
                                ("tests", &ts.tests().to_string()),
                                ("errors", &ts.errors().to_string()),
                                ("failures", &ts.failures().to_string()),
                                ("hostname", &ts.hostname),
                                ("timestamp", &ts.timestamp.format(&Rfc3339).unwrap()),
                                ("time", &ts.time().as_seconds_f64().to_string()),
                            ])
                            .write_empty_or_inner(
                                |_| {
                                    ts.testcases.is_empty()
                                        && ts.system_out.is_none()
                                        && ts.system_err.is_none()
                                },
                                |w| {
                                    w.write_iter(ts.testcases.iter(), |w, tc| tc.write_xml(w))?
                                        .write_opt(ts.system_out.as_ref(), |writer, out| {
                                            writer
                                                .create_element("system-out")
                                                .write_cdata_content(BytesCData::new(out))
                                        })?
                                        .write_opt(ts.system_err.as_ref(), |writer, err| {
                                            writer
                                                .create_element("system-err")
                                                .write_cdata_content(BytesCData::new(err))
                                        })
                                        .map(drop)
                                },
                            )
                    })
                    .map(drop)
                },
            )
            .map(drop)
    }
}

impl TestCase {
    /// Write the XML version of the [`TestCase`] to the given [`Writer`].
    fn write_xml<'a, W: Write>(&self, w: &'a mut Writer<W>) -> Result<&'a mut Writer<W>> {
        let time = self.time.as_seconds_f64().to_string();
        w.create_element("testcase")
            .with_attributes(
                [
                    Some(("name", self.name.as_str())),
                    Some(("time", time.as_str())),
                    self.classname.as_ref().map(|cl| ("classname", cl.as_str())),
                    self.filepath.as_ref().map(|f| ("file", f.as_str())),
                ]
                .into_iter()
                .flatten(),
            )
            .write_empty_or_inner(
                |_| {
                    matches!(self.result, TestResult::Success)
                        && self.system_out.is_none()
                        && self.system_err.is_none()
                },
                |w| {
                    match self.result {
                        TestResult::Success => Ok(w),
                        TestResult::Error {
                            ref type_,
                            ref message,
                            ref cause,
                        } => w
                            .create_element("error")
                            .with_attributes([
                                ("type", type_.as_str()),
                                ("message", message.as_str()),
                            ])
                            .write_empty_or_inner(
                                |_| cause.is_none(),
                                |w| {
                                    w.write_opt(cause.as_ref(), |w, cause| {
                                        let data = BytesCData::new(cause.as_str());
                                        w.write_event(Event::CData(BytesCData::new(
                                            String::from_utf8_lossy(&data),
                                        )))
                                        .map(|_| w)
                                    })
                                    .map(drop)
                                },
                            ),
                        TestResult::Failure {
                            ref type_,
                            ref message,
                            ref cause,
                        } => w
                            .create_element("failure")
                            .with_attributes([
                                ("type", type_.as_str()),
                                ("message", message.as_str()),
                            ])
                            .write_empty_or_inner(
                                |_| cause.is_none(),
                                |w| {
                                    w.write_opt(cause.as_ref(), |w, cause| {
                                        let data = BytesCData::new(cause.as_str());
                                        w.write_event(Event::CData(BytesCData::new(
                                            String::from_utf8_lossy(&data),
                                        )))
                                        .map(|_| w)
                                    })
                                    .map(drop)
                                },
                            ),
                        TestResult::Skipped => w.create_element("skipped").write_empty(),
                    }?
                    .write_opt(self.system_out.as_ref(), |w, out| {
                        w.create_element("system-out")
                            .write_cdata_content(BytesCData::new(out.as_str()))
                    })?
                    .write_opt(self.system_err.as_ref(), |w, err| {
                        w.create_element("system-err")
                            .write_cdata_content(BytesCData::new(err.as_str()))
                    })
                    .map(drop)
                },
            )
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

/// [`Writer`] extension.
trait WriterExt {
    /// [`Write`]s in case `val` is [`Some`] or does nothing otherwise.
    fn write_opt<T>(
        &mut self,
        val: Option<T>,
        inner: impl FnOnce(&mut Self, T) -> Result<&mut Self>,
    ) -> Result<&mut Self>;

    /// [`Write`]s every item of the [`Iterator`].
    fn write_iter<T, I>(
        &mut self,
        val: I,
        inner: impl FnMut(&mut Self, T) -> Result<&mut Self>,
    ) -> Result<&mut Self>
    where
        I: IntoIterator<Item = T>;
}

impl<W: Write> WriterExt for Writer<W> {
    fn write_opt<T>(
        &mut self,
        val: Option<T>,
        inner: impl FnOnce(&mut Self, T) -> Result<&mut Self>,
    ) -> Result<&mut Self> {
        if let Some(val) = val {
            inner(self, val)
        } else {
            Ok(self)
        }
    }

    fn write_iter<T, I>(
        &mut self,
        iter: I,
        inner: impl FnMut(&mut Self, T) -> Result<&mut Self>,
    ) -> Result<&mut Self>
    where
        I: IntoIterator<Item = T>,
    {
        iter.into_iter().try_fold(self, inner)
    }
}

/// [`ElementWriter`] extension.
trait ElementWriterExt<'a, W: Write> {
    /// [`Writes`] with `inner` in case `is_empty` resolves to [`false`] or
    /// [`Write`]s with [`ElementWriter::write_empty`] otherwise.
    fn write_empty_or_inner<Inner>(
        self,
        is_empty: impl FnOnce(&mut Self) -> bool,
        inner: Inner,
    ) -> Result<&'a mut Writer<W>>
    where
        Inner: Fn(&mut Writer<W>) -> Result<()>;
}

impl<'a, W: Write> ElementWriterExt<'a, W> for ElementWriter<'a, W> {
    fn write_empty_or_inner<Inner>(
        mut self,
        is_empty: impl FnOnce(&mut Self) -> bool,
        inner: Inner,
    ) -> Result<&'a mut Writer<W>>
    where
        Inner: Fn(&mut Writer<W>) -> Result<()>,
    {
        if is_empty(&mut self) {
            self.write_empty()
        } else {
            self.write_inner_content(inner)
        }
    }
}
