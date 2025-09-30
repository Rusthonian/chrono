use pyo3::prelude::*;
use chrono::{Utc, Local};
use crate::datetime::{PyDateTimeUtc, PyDateTimeLocal};

#[pyfunction]
pub fn now_utc() -> PyDateTimeUtc {
    PyDateTimeUtc { inner: Utc::now() }
}

#[pyfunction]
pub fn now_local() -> PyDateTimeLocal {
    PyDateTimeLocal { inner: Local::now() }
}

#[pyfunction]
pub fn timestamp_millis(millis: i64) -> PyResult<PyDateTimeUtc> {
    chrono::DateTime::from_timestamp_millis(millis)
        .map(|dt| PyDateTimeUtc { inner: dt })
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid timestamp"))
}

#[pyfunction]
pub fn timestamp_micros(micros: i64) -> PyResult<PyDateTimeUtc> {
    chrono::DateTime::from_timestamp_micros(micros)
        .map(|dt| PyDateTimeUtc { inner: dt })
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid timestamp"))
}

#[pyfunction]
pub fn timestamp_nanos(nanos: i64) -> PyResult<PyDateTimeUtc> {
    Ok(PyDateTimeUtc {
        inner: chrono::DateTime::from_timestamp_nanos(nanos)
    })
}
