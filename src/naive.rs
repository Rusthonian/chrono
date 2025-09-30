use pyo3::prelude::*;
use chrono::{NaiveDateTime, NaiveDate, NaiveTime, Datelike, Timelike};
use crate::duration::PyDuration;
use crate::weekday::PyWeekday;
use crate::errors::ChronoError;

#[pyclass(name = "NaiveDateTime")]
#[derive(Clone, Copy)]
pub struct PyNaiveDateTime {
    pub inner: NaiveDateTime,
}

#[pymethods]
impl PyNaiveDateTime {
    #[new]
    #[pyo3(signature = (year, month, day, hour, minute, second, nano=None))]
    fn new(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32, nano: Option<u32>) -> PyResult<Self> {
        let date = NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| ChronoError::OutOfRange("Invalid date".to_string()))?;
        let time = NaiveTime::from_hms_nano_opt(hour, minute, second, nano.unwrap_or(0))
            .ok_or_else(|| ChronoError::OutOfRange("Invalid time".to_string()))?;
        Ok(PyNaiveDateTime {
            inner: NaiveDateTime::new(date, time),
        })
    }

    #[staticmethod]
    fn from_timestamp(secs: i64, nsecs: u32) -> PyResult<Self> {
        chrono::DateTime::from_timestamp(secs, nsecs)
            .map(|dt| PyNaiveDateTime { inner: dt.naive_utc() })
            .ok_or_else(|| ChronoError::OutOfRange("Timestamp out of range".to_string()).into())
    }

    #[staticmethod]
    fn from_timestamp_millis(millis: i64) -> PyResult<Self> {
        chrono::DateTime::from_timestamp_millis(millis)
            .map(|dt| PyNaiveDateTime { inner: dt.naive_utc() })
            .ok_or_else(|| ChronoError::OutOfRange("Timestamp out of range".to_string()).into())
    }

    #[staticmethod]
    fn from_timestamp_micros(micros: i64) -> PyResult<Self> {
        chrono::DateTime::from_timestamp_micros(micros)
            .map(|dt| PyNaiveDateTime { inner: dt.naive_utc() })
            .ok_or_else(|| ChronoError::OutOfRange("Timestamp out of range".to_string()).into())
    }

    #[staticmethod]
    fn parse_from_str(s: &str, fmt: &str) -> PyResult<Self> {
        NaiveDateTime::parse_from_str(s, fmt)
            .map(|inner| PyNaiveDateTime { inner })
            .map_err(|e| ChronoError::from(e).into())
    }

    fn date(&self) -> PyNaiveDate {
        PyNaiveDate { inner: self.inner.date() }
    }

    fn time(&self) -> PyNaiveTime {
        PyNaiveTime { inner: self.inner.time() }
    }

    fn timestamp(&self) -> i64 {
        self.inner.and_utc().timestamp()
    }

    fn timestamp_millis(&self) -> i64 {
        self.inner.and_utc().timestamp_millis()
    }

    fn timestamp_micros(&self) -> i64 {
        self.inner.and_utc().timestamp_micros()
    }

    fn timestamp_nanos_opt(&self) -> Option<i64> {
        Some(self.inner.and_utc().timestamp_nanos_opt()?)
    }

    fn year(&self) -> i32 {
        self.inner.year()
    }

    fn month(&self) -> u32 {
        self.inner.month()
    }

    fn month0(&self) -> u32 {
        self.inner.month0()
    }

    fn day(&self) -> u32 {
        self.inner.day()
    }

    fn day0(&self) -> u32 {
        self.inner.day0()
    }

    fn ordinal(&self) -> u32 {
        self.inner.ordinal()
    }

    fn ordinal0(&self) -> u32 {
        self.inner.ordinal0()
    }

    fn weekday(&self) -> PyWeekday {
        PyWeekday { inner: self.inner.weekday() }
    }

    fn iso_week(&self) -> u32 {
        self.inner.iso_week().week()
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

    fn format(&self, fmt: &str) -> String {
        self.inner.format(fmt).to_string()
    }

    fn __add__(&self, rhs: &PyDuration) -> Self {
        PyNaiveDateTime {
            inner: self.inner + rhs.inner,
        }
    }

    fn __sub__(&self, rhs: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(duration) = rhs.downcast::<PyDuration>() {
                Ok(PyNaiveDateTime {
                    inner: self.inner - duration.borrow().inner,
                }.into_py(py))
            } else if let Ok(other) = rhs.downcast::<PyNaiveDateTime>() {
                let diff = self.inner.signed_duration_since(other.borrow().inner);
                Ok(PyDuration {
                    inner: diff,
                }.into_py(py))
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "Can only subtract Duration or NaiveDateTime"
                ))
            }
        })
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("NaiveDateTime({})", self.inner)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.inner != other.inner
    }

    fn __lt__(&self, other: &Self) -> bool {
        self.inner < other.inner
    }

    fn __le__(&self, other: &Self) -> bool {
        self.inner <= other.inner
    }

    fn __gt__(&self, other: &Self) -> bool {
        self.inner > other.inner
    }

    fn __ge__(&self, other: &Self) -> bool {
        self.inner >= other.inner
    }

    fn __hash__(&self) -> u64 {
        self.inner.and_utc().timestamp() as u64
    }
}

#[pyclass(name = "NaiveDate")]
#[derive(Clone, Copy)]
pub struct PyNaiveDate {
    pub inner: NaiveDate,
}

#[pymethods]
impl PyNaiveDate {
    #[new]
    fn new(year: i32, month: u32, day: u32) -> PyResult<Self> {
        NaiveDate::from_ymd_opt(year, month, day)
            .map(|inner| PyNaiveDate { inner })
            .ok_or_else(|| ChronoError::OutOfRange("Invalid date".to_string()).into())
    }

    #[staticmethod]
    fn from_ymd(year: i32, month: u32, day: u32) -> PyResult<Self> {
        NaiveDate::from_ymd_opt(year, month, day)
            .map(|inner| PyNaiveDate { inner })
            .ok_or_else(|| ChronoError::OutOfRange("Invalid date".to_string()).into())
    }

    #[staticmethod]
    fn from_yo(year: i32, ordinal: u32) -> PyResult<Self> {
        NaiveDate::from_yo_opt(year, ordinal)
            .map(|inner| PyNaiveDate { inner })
            .ok_or_else(|| ChronoError::OutOfRange("Invalid ordinal date".to_string()).into())
    }

    #[staticmethod]
    fn from_isoywd(year: i32, week: u32, weekday: u32) -> PyResult<Self> {
        let wd = match weekday {
            0 => chrono::Weekday::Mon,
            1 => chrono::Weekday::Tue,
            2 => chrono::Weekday::Wed,
            3 => chrono::Weekday::Thu,
            4 => chrono::Weekday::Fri,
            5 => chrono::Weekday::Sat,
            6 => chrono::Weekday::Sun,
            _ => return Err(ChronoError::OutOfRange("Weekday must be 0-6".to_string()).into()),
        };
        NaiveDate::from_isoywd_opt(year, week, wd)
            .map(|inner| PyNaiveDate { inner })
            .ok_or_else(|| ChronoError::OutOfRange("Invalid ISO week date".to_string()).into())
    }

    #[staticmethod]
    fn from_num_days_from_ce(days: i32) -> Self {
        PyNaiveDate {
            inner: NaiveDate::from_num_days_from_ce_opt(days).unwrap(),
        }
    }

    #[staticmethod]
    fn parse_from_str(s: &str, fmt: &str) -> PyResult<Self> {
        NaiveDate::parse_from_str(s, fmt)
            .map(|inner| PyNaiveDate { inner })
            .map_err(|e| ChronoError::from(e).into())
    }

    fn and_time(&self, time: &PyNaiveTime) -> PyNaiveDateTime {
        PyNaiveDateTime {
            inner: self.inner.and_time(time.inner),
        }
    }

    fn and_hms(& self, hour: u32, min: u32, sec: u32) -> PyResult<PyNaiveDateTime> {
        self.inner.and_hms_opt(hour, min, sec)
            .map(|inner| PyNaiveDateTime { inner })
            .ok_or_else(|| ChronoError::OutOfRange("Invalid time".to_string()).into())
    }

    fn and_hms_milli(&self, hour: u32, min: u32, sec: u32, milli: u32) -> PyResult<PyNaiveDateTime> {
        self.inner.and_hms_milli_opt(hour, min, sec, milli)
            .map(|inner| PyNaiveDateTime { inner })
            .ok_or_else(|| ChronoError::OutOfRange("Invalid time".to_string()).into())
    }

    fn and_hms_micro(&self, hour: u32, min: u32, sec: u32, micro: u32) -> PyResult<PyNaiveDateTime> {
        self.inner.and_hms_micro_opt(hour, min, sec, micro)
            .map(|inner| PyNaiveDateTime { inner })
            .ok_or_else(|| ChronoError::OutOfRange("Invalid time".to_string()).into())
    }

    fn and_hms_nano(&self, hour: u32, min: u32, sec: u32, nano: u32) -> PyResult<PyNaiveDateTime> {
        self.inner.and_hms_nano_opt(hour, min, sec, nano)
            .map(|inner| PyNaiveDateTime { inner })
            .ok_or_else(|| ChronoError::OutOfRange("Invalid time".to_string()).into())
    }

    fn year(&self) -> i32 {
        self.inner.year()
    }

    fn month(&self) -> u32 {
        self.inner.month()
    }

    fn month0(&self) -> u32 {
        self.inner.month0()
    }

    fn day(&self) -> u32 {
        self.inner.day()
    }

    fn day0(&self) -> u32 {
        self.inner.day0()
    }

    fn ordinal(&self) -> u32 {
        self.inner.ordinal()
    }

    fn ordinal0(&self) -> u32 {
        self.inner.ordinal0()
    }

    fn weekday(&self) -> PyWeekday {
        PyWeekday { inner: self.inner.weekday() }
    }

    fn iso_week(&self) -> u32 {
        self.inner.iso_week().week()
    }

    fn num_days_from_ce(&self) -> i32 {
        self.inner.num_days_from_ce()
    }

    fn succ(&self) -> Self {
        PyNaiveDate { inner: self.inner.succ_opt().unwrap() }
    }

    fn pred(&self) -> Self {
        PyNaiveDate { inner: self.inner.pred_opt().unwrap() }
    }

    fn format(&self, fmt: &str) -> String {
        self.inner.format(fmt).to_string()
    }

    fn __add__(&self, rhs: &PyDuration) -> Self {
        PyNaiveDate {
            inner: self.inner + rhs.inner,
        }
    }

    fn __sub__(&self, rhs: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(duration) = rhs.downcast::<PyDuration>() {
                Ok(PyNaiveDate {
                    inner: self.inner - duration.borrow().inner,
                }.into_py(py))
            } else if let Ok(other) = rhs.downcast::<PyNaiveDate>() {
                let diff = self.inner.signed_duration_since(other.borrow().inner);
                Ok(PyDuration {
                    inner: diff,
                }.into_py(py))
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "Can only subtract Duration or NaiveDate"
                ))
            }
        })
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("NaiveDate({})", self.inner)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.inner != other.inner
    }

    fn __lt__(&self, other: &Self) -> bool {
        self.inner < other.inner
    }

    fn __le__(&self, other: &Self) -> bool {
        self.inner <= other.inner
    }

    fn __gt__(&self, other: &Self) -> bool {
        self.inner > other.inner
    }

    fn __ge__(&self, other: &Self) -> bool {
        self.inner >= other.inner
    }

    fn __hash__(&self) -> u64 {
        self.inner.num_days_from_ce() as u64
    }
}

#[pyclass(name = "NaiveTime")]
#[derive(Clone, Copy)]
pub struct PyNaiveTime {
    pub inner: NaiveTime,
}

#[pymethods]
impl PyNaiveTime {
    #[new]
    #[pyo3(signature = (hour, minute, second, nano=None))]
    fn new(hour: u32, minute: u32, second: u32, nano: Option<u32>) -> PyResult<Self> {
        NaiveTime::from_hms_nano_opt(hour, minute, second, nano.unwrap_or(0))
            .map(|inner| PyNaiveTime { inner })
            .ok_or_else(|| ChronoError::OutOfRange("Invalid time".to_string()).into())
    }

    #[staticmethod]
    fn from_hms(hour: u32, min: u32, sec: u32) -> PyResult<Self> {
        NaiveTime::from_hms_opt(hour, min, sec)
            .map(|inner| PyNaiveTime { inner })
            .ok_or_else(|| ChronoError::OutOfRange("Invalid time".to_string()).into())
    }

    #[staticmethod]
    fn from_hms_milli(hour: u32, min: u32, sec: u32, milli: u32) -> PyResult<Self> {
        NaiveTime::from_hms_milli_opt(hour, min, sec, milli)
            .map(|inner| PyNaiveTime { inner })
            .ok_or_else(|| ChronoError::OutOfRange("Invalid time".to_string()).into())
    }

    #[staticmethod]
    fn from_hms_micro(hour: u32, min: u32, sec: u32, micro: u32) -> PyResult<Self> {
        NaiveTime::from_hms_micro_opt(hour, min, sec, micro)
            .map(|inner| PyNaiveTime { inner })
            .ok_or_else(|| ChronoError::OutOfRange("Invalid time".to_string()).into())
    }

    #[staticmethod]
    fn from_hms_nano(hour: u32, min: u32, sec: u32, nano: u32) -> PyResult<Self> {
        NaiveTime::from_hms_nano_opt(hour, min, sec, nano)
            .map(|inner| PyNaiveTime { inner })
            .ok_or_else(|| ChronoError::OutOfRange("Invalid time".to_string()).into())
    }

    #[staticmethod]
    fn from_num_seconds_from_midnight(secs: u32, nano: u32) -> PyResult<Self> {
        NaiveTime::from_num_seconds_from_midnight_opt(secs, nano)
            .map(|inner| PyNaiveTime { inner })
            .ok_or_else(|| ChronoError::OutOfRange("Invalid time".to_string()).into())
    }

    #[staticmethod]
    fn parse_from_str(s: &str, fmt: &str) -> PyResult<Self> {
        NaiveTime::parse_from_str(s, fmt)
            .map(|inner| PyNaiveTime { inner })
            .map_err(|e| ChronoError::from(e).into())
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

    fn num_seconds_from_midnight(&self) -> u32 {
        self.inner.num_seconds_from_midnight()
    }

    fn format(&self, fmt: &str) -> String {
        self.inner.format(fmt).to_string()
    }

    fn __add__(&self, rhs: &PyDuration) -> Self {
        PyNaiveTime {
            inner: self.inner + rhs.inner,
        }
    }

    fn __sub__(&self, rhs: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(duration) = rhs.downcast::<PyDuration>() {
                Ok(PyNaiveTime {
                    inner: self.inner - duration.borrow().inner,
                }.into_py(py))
            } else if let Ok(other) = rhs.downcast::<PyNaiveTime>() {
                let diff = self.inner.signed_duration_since(other.borrow().inner);
                Ok(PyDuration {
                    inner: diff,
                }.into_py(py))
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "Can only subtract Duration or NaiveTime"
                ))
            }
        })
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("NaiveTime({})", self.inner)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.inner != other.inner
    }

    fn __lt__(&self, other: &Self) -> bool {
        self.inner < other.inner
    }

    fn __le__(&self, other: &Self) -> bool {
        self.inner <= other.inner
    }

    fn __gt__(&self, other: &Self) -> bool {
        self.inner > other.inner
    }

    fn __ge__(&self, other: &Self) -> bool {
        self.inner >= other.inner
    }

    fn __hash__(&self) -> u64 {
        self.inner.num_seconds_from_midnight() as u64
    }
}
