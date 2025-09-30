use pyo3::prelude::*;
use chrono::{DateTime, NaiveDateTime, NaiveDate, NaiveTime};
use crate::datetime::PyDateTime;
use crate::naive::{PyNaiveDateTime, PyNaiveDate, PyNaiveTime};
use crate::errors::ChronoError;

#[pyfunction]
pub fn parse_datetime(s: &str, fmt: &str) -> PyResult<PyNaiveDateTime> {
    NaiveDateTime::parse_from_str(s, fmt)
        .map(|inner| PyNaiveDateTime { inner })
        .map_err(|e| ChronoError::from(e).into())
}

#[pyfunction]
pub fn parse_date(s: &str, fmt: &str) -> PyResult<PyNaiveDate> {
    NaiveDate::parse_from_str(s, fmt)
        .map(|inner| PyNaiveDate { inner })
        .map_err(|e| ChronoError::from(e).into())
}

#[pyfunction]
pub fn parse_time(s: &str, fmt: &str) -> PyResult<PyNaiveTime> {
    NaiveTime::parse_from_str(s, fmt)
        .map(|inner| PyNaiveTime { inner })
        .map_err(|e| ChronoError::from(e).into())
}

#[pyfunction]
pub fn parse_from_str(s: &str, fmt: &str) -> PyResult<PyNaiveDateTime> {
    NaiveDateTime::parse_from_str(s, fmt)
        .map(|inner| PyNaiveDateTime { inner })
        .map_err(|e| ChronoError::from(e).into())
}

#[pyfunction]
pub fn parse_from_rfc3339(s: &str) -> PyResult<PyDateTime> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| PyDateTime { utc: dt.with_timezone(&chrono::Utc) })
        .map_err(|e| ChronoError::from(e).into())
}

#[pyfunction]
pub fn parse_from_rfc2822(s: &str) -> PyResult<PyDateTime> {
    DateTime::parse_from_rfc2822(s)
        .map(|dt| PyDateTime { utc: dt.with_timezone(&chrono::Utc) })
        .map_err(|e| ChronoError::from(e).into())
}

#[pyfunction]
pub fn format_rfc3339(dt: &PyDateTime) -> String {
    dt.utc.to_rfc3339()
}

#[pyfunction]
pub fn format_rfc2822(dt: &PyDateTime) -> String {
    dt.utc.to_rfc2822()
}
