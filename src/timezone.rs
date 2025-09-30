use pyo3::prelude::*;
use chrono::FixedOffset;

#[pyclass(name = "Utc")]
#[derive(Clone, Copy)]
pub struct PyUtc;

#[pymethods]
impl PyUtc {
    #[new]
    fn new() -> Self {
        PyUtc
    }

    fn __str__(&self) -> String {
        "UTC".to_string()
    }

    fn __repr__(&self) -> String {
        "Utc()".to_string()
    }
}

#[pyclass(name = "Local")]
#[derive(Clone, Copy)]
pub struct PyLocal;

#[pymethods]
impl PyLocal {
    #[new]
    fn new() -> Self {
        PyLocal
    }

    fn __str__(&self) -> String {
        "Local".to_string()
    }

    fn __repr__(&self) -> String {
        "Local()".to_string()
    }
}

#[pyclass(name = "FixedOffset")]
#[derive(Clone, Copy)]
pub struct PyFixedOffset {
    pub inner: FixedOffset,
}

#[pymethods]
impl PyFixedOffset {
    #[new]
    fn new(seconds: i32) -> Self {
        PyFixedOffset {
            inner: FixedOffset::east_opt(seconds).unwrap_or(FixedOffset::east_opt(0).unwrap()),
        }
    }

    #[staticmethod]
    fn east(seconds: i32) -> PyResult<Self> {
        FixedOffset::east_opt(seconds)
            .map(|inner| PyFixedOffset { inner })
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid offset"))
    }

    #[staticmethod]
    fn east_opt(seconds: i32) -> Option<Self> {
        FixedOffset::east_opt(seconds).map(|inner| PyFixedOffset { inner })
    }

    #[staticmethod]
    fn west(seconds: i32) -> PyResult<Self> {
        FixedOffset::west_opt(seconds)
            .map(|inner| PyFixedOffset { inner })
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid offset"))
    }

    #[staticmethod]
    fn west_opt(seconds: i32) -> Option<Self> {
        FixedOffset::west_opt(seconds).map(|inner| PyFixedOffset { inner })
    }

    fn local_minus_utc(&self) -> i32 {
        self.inner.local_minus_utc()
    }

    fn utc_minus_local(&self) -> i32 {
        self.inner.utc_minus_local()
    }

    fn __str__(&self) -> String {
        format!("{:+}", self.inner)
    }

    fn __repr__(&self) -> String {
        format!("FixedOffset(seconds={})", self.inner.local_minus_utc())
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.inner != other.inner
    }

    fn __hash__(&self) -> u64 {
        self.inner.local_minus_utc() as u64
    }
}
