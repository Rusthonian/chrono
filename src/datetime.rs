use pyo3::prelude::*;
use chrono::{DateTime, Utc, Local, FixedOffset, Datelike, Timelike};
use crate::duration::PyDuration;
use crate::naive::{PyNaiveDateTime, PyNaiveDate, PyNaiveTime};
use crate::timezone::PyFixedOffset;
use crate::weekday::PyWeekday;
use crate::errors::ChronoError;

#[pyclass(name = "DateTime")]
#[derive(Clone)]
pub struct PyDateTime {
    pub utc: DateTime<Utc>,
}

#[pymethods]
impl PyDateTime {
    #[new]
    #[pyo3(signature = (year, month, day, hour, minute, second, nano=None))]
    fn new(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32, nano: Option<u32>) -> PyResult<Self> {
        let naive = chrono::NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| ChronoError::OutOfRange("Invalid date".to_string()))?
            .and_hms_nano_opt(hour, minute, second, nano.unwrap_or(0))
            .ok_or_else(|| ChronoError::OutOfRange("Invalid time".to_string()))?;
        
        Ok(PyDateTime {
            utc: DateTime::from_naive_utc_and_offset(naive, Utc),
        })
    }

    #[staticmethod]
    fn now() -> Self {
        PyDateTime { utc: Utc::now() }
    }

    #[staticmethod]
    fn from_timestamp(secs: i64, nsecs: u32) -> PyResult<Self> {
        DateTime::from_timestamp(secs, nsecs)
            .map(|dt| PyDateTime { utc: dt })
            .ok_or_else(|| ChronoError::OutOfRange("Timestamp out of range".to_string()).into())
    }

    #[staticmethod]
    fn from_timestamp_millis(millis: i64) -> PyResult<Self> {
        DateTime::from_timestamp_millis(millis)
            .map(|dt| PyDateTime { utc: dt })
            .ok_or_else(|| ChronoError::OutOfRange("Timestamp out of range".to_string()).into())
    }

    #[staticmethod]
    fn from_timestamp_micros(micros: i64) -> PyResult<Self> {
        DateTime::from_timestamp_micros(micros)
            .map(|dt| PyDateTime { utc: dt })
            .ok_or_else(|| ChronoError::OutOfRange("Timestamp out of range".to_string()).into())
    }

    #[staticmethod]
    fn parse_from_rfc3339(s: &str) -> PyResult<Self> {
        DateTime::parse_from_rfc3339(s)
            .map(|dt| PyDateTime { utc: dt.with_timezone(&Utc) })
            .map_err(|e| ChronoError::from(e).into())
    }

    #[staticmethod]
    fn parse_from_rfc2822(s: &str) -> PyResult<Self> {
        DateTime::parse_from_rfc2822(s)
            .map(|dt| PyDateTime { utc: dt.with_timezone(&Utc) })
            .map_err(|e| ChronoError::from(e).into())
    }

    fn to_utc(&self) -> PyDateTimeUtc {
        PyDateTimeUtc { inner: self.utc }
    }

    fn to_local(&self) -> PyDateTimeLocal {
        PyDateTimeLocal { inner: self.utc.with_timezone(&Local) }
    }

    fn to_fixed(&self, offset: &PyFixedOffset) -> PyDateTimeFixed {
        PyDateTimeFixed { inner: self.utc.with_timezone(&offset.inner) }
    }

    fn naive_utc(&self) -> PyNaiveDateTime {
        PyNaiveDateTime { inner: self.utc.naive_utc() }
    }

    fn naive_local(&self) -> PyNaiveDateTime {
        PyNaiveDateTime { inner: self.utc.naive_local() }
    }

    fn timestamp(&self) -> i64 {
        self.utc.timestamp()
    }

    fn timestamp_millis(&self) -> i64 {
        self.utc.timestamp_millis()
    }

    fn timestamp_micros(&self) -> i64 {
        self.utc.timestamp_micros()
    }

    fn timestamp_nanos_opt(&self) -> Option<i64> {
        self.utc.timestamp_nanos_opt()
    }

    fn year(&self) -> i32 {
        self.utc.year()
    }

    fn month(&self) -> u32 {
        self.utc.month()
    }

    fn day(&self) -> u32 {
        self.utc.day()
    }

    fn hour(&self) -> u32 {
        self.utc.hour()
    }

    fn minute(&self) -> u32 {
        self.utc.minute()
    }

    fn second(&self) -> u32 {
        self.utc.second()
    }

    fn nanosecond(&self) -> u32 {
        self.utc.nanosecond()
    }

    fn weekday(&self) -> PyWeekday {
        PyWeekday { inner: self.utc.weekday() }
    }

    fn ordinal(&self) -> u32 {
        self.utc.ordinal()
    }

    fn iso_week(&self) -> u32 {
        self.utc.iso_week().week()
    }

    fn date(&self) -> PyNaiveDate {
        PyNaiveDate { inner: self.utc.date_naive() }
    }

    fn time(&self) -> PyNaiveTime {
        PyNaiveTime { inner: self.utc.time() }
    }

    fn to_rfc3339(&self) -> String {
        self.utc.to_rfc3339()
    }

    fn to_rfc2822(&self) -> String {
        self.utc.to_rfc2822()
    }

    fn format(&self, fmt: &str) -> String {
        self.utc.format(fmt).to_string()
    }

    fn __add__(&self, rhs: &PyDuration) -> Self {
        PyDateTime {
            utc: self.utc + rhs.inner,
        }
    }

    fn __sub__(&self, rhs: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(duration) = rhs.downcast::<PyDuration>() {
                Ok(PyDateTime {
                    utc: self.utc - duration.borrow().inner,
                }.into_py(py))
            } else if let Ok(other) = rhs.downcast::<PyDateTime>() {
                let diff = self.utc.signed_duration_since(other.borrow().utc);
                Ok(PyDuration {
                    inner: diff,
                }.into_py(py))
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "Can only subtract Duration or DateTime"
                ))
            }
        })
    }

    fn __str__(&self) -> String {
        self.utc.to_string()
    }

    fn __repr__(&self) -> String {
        format!("DateTime({})", self.utc.to_rfc3339())
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.utc == other.utc
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.utc != other.utc
    }

    fn __lt__(&self, other: &Self) -> bool {
        self.utc < other.utc
    }

    fn __le__(&self, other: &Self) -> bool {
        self.utc <= other.utc
    }

    fn __gt__(&self, other: &Self) -> bool {
        self.utc > other.utc
    }

    fn __ge__(&self, other: &Self) -> bool {
        self.utc >= other.utc
    }

    fn __hash__(&self) -> u64 {
        self.utc.timestamp() as u64
    }
}

#[pyclass(name = "DateTimeUtc")]
#[derive(Clone, Copy)]
pub struct PyDateTimeUtc {
    pub inner: DateTime<Utc>,
}

#[pymethods]
impl PyDateTimeUtc {
    #[new]
    #[pyo3(signature = (year, month, day, hour, minute, second, nano=None))]
    fn new(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32, nano: Option<u32>) -> PyResult<Self> {
        let naive = chrono::NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| ChronoError::OutOfRange("Invalid date".to_string()))?
            .and_hms_nano_opt(hour, minute, second, nano.unwrap_or(0))
            .ok_or_else(|| ChronoError::OutOfRange("Invalid time".to_string()))?;
        
        Ok(PyDateTimeUtc {
            inner: DateTime::from_naive_utc_and_offset(naive, Utc),
        })
    }

    #[staticmethod]
    fn now() -> Self {
        PyDateTimeUtc { inner: Utc::now() }
    }

    #[staticmethod]
    fn from_timestamp(secs: i64, nsecs: u32) -> PyResult<Self> {
        DateTime::from_timestamp(secs, nsecs)
            .map(|dt| PyDateTimeUtc { inner: dt })
            .ok_or_else(|| ChronoError::OutOfRange("Timestamp out of range".to_string()).into())
    }

    fn timestamp(&self) -> i64 {
        self.inner.timestamp()
    }

    fn timestamp_millis(&self) -> i64 {
        self.inner.timestamp_millis()
    }

    fn timestamp_micros(&self) -> i64 {
        self.inner.timestamp_micros()
    }

    fn year(&self) -> i32 {
        self.inner.year()
    }

    fn month(&self) -> u32 {
        self.inner.month()
    }

    fn day(&self) -> u32 {
        self.inner.day()
    }

    fn hour(&self) -> u32 {
        self.inner.hour()
    }

    fn minute(&self) -> u32 {
        self.inner.minute()
    }

    fn second(&self) -> u32 {
        self.inner.second()
    }

    fn nanosecond(&self) -> u32 {
        self.inner.nanosecond()
    }

    fn weekday(&self) -> PyWeekday {
        PyWeekday { inner: self.inner.weekday() }
    }

    fn to_rfc3339(&self) -> String {
        self.inner.to_rfc3339()
    }

    fn format(&self, fmt: &str) -> String {
        self.inner.format(fmt).to_string()
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("DateTimeUtc({})", self.inner.to_rfc3339())
    }
}

#[pyclass(name = "DateTimeLocal")]
#[derive(Clone, Copy)]
pub struct PyDateTimeLocal {
    pub inner: DateTime<Local>,
}

#[pymethods]
impl PyDateTimeLocal {
    #[staticmethod]
    fn now() -> Self {
        PyDateTimeLocal { inner: Local::now() }
    }

    fn timestamp(&self) -> i64 {
        self.inner.timestamp()
    }

    fn timestamp_millis(&self) -> i64 {
        self.inner.timestamp_millis()
    }

    fn year(&self) -> i32 {
        self.inner.year()
    }

    fn month(&self) -> u32 {
        self.inner.month()
    }

    fn day(&self) -> u32 {
        self.inner.day()
    }

    fn hour(&self) -> u32 {
        self.inner.hour()
    }

    fn minute(&self) -> u32 {
        self.inner.minute()
    }

    fn second(&self) -> u32 {
        self.inner.second()
    }

    fn to_rfc3339(&self) -> String {
        self.inner.to_rfc3339()
    }

    fn format(&self, fmt: &str) -> String {
        self.inner.format(fmt).to_string()
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("DateTimeLocal({})", self.inner.to_rfc3339())
    }
}

#[pyclass(name = "DateTimeFixed")]
#[derive(Clone, Copy)]
pub struct PyDateTimeFixed {
    pub inner: DateTime<FixedOffset>,
}

#[pymethods]
impl PyDateTimeFixed {
    fn timestamp(&self) -> i64 {
        self.inner.timestamp()
    }

    fn year(&self) -> i32 {
        self.inner.year()
    }

    fn month(&self) -> u32 {
        self.inner.month()
    }

    fn day(&self) -> u32 {
        self.inner.day()
    }

    fn hour(&self) -> u32 {
        self.inner.hour()
    }

    fn minute(&self) -> u32 {
        self.inner.minute()
    }

    fn second(&self) -> u32 {
        self.inner.second()
    }

    fn offset_seconds(&self) -> i32 {
        self.inner.offset().local_minus_utc()
    }

    fn to_rfc3339(&self) -> String {
        self.inner.to_rfc3339()
    }

    fn format(&self, fmt: &str) -> String {
        self.inner.format(fmt).to_string()
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("DateTimeFixed({})", self.inner.to_rfc3339())
    }
}
