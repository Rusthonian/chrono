use pyo3::prelude::*;
use chrono::Month;

#[pyclass(name = "Month")]
#[derive(Clone, Copy)]
pub struct PyMonth {
    pub inner: Month,
}

#[pymethods]
impl PyMonth {
    #[new]
    fn new(month: u32) -> PyResult<Self> {
        let m = match month {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Month must be 1-12")),
        };
        Ok(PyMonth { inner: m })
    }

    #[staticmethod]
    fn january() -> Self {
        PyMonth { inner: Month::January }
    }

    #[staticmethod]
    fn february() -> Self {
        PyMonth { inner: Month::February }
    }

    #[staticmethod]
    fn march() -> Self {
        PyMonth { inner: Month::March }
    }

    #[staticmethod]
    fn april() -> Self {
        PyMonth { inner: Month::April }
    }

    #[staticmethod]
    fn may() -> Self {
        PyMonth { inner: Month::May }
    }

    #[staticmethod]
    fn june() -> Self {
        PyMonth { inner: Month::June }
    }

    #[staticmethod]
    fn july() -> Self {
        PyMonth { inner: Month::July }
    }

    #[staticmethod]
    fn august() -> Self {
        PyMonth { inner: Month::August }
    }

    #[staticmethod]
    fn september() -> Self {
        PyMonth { inner: Month::September }
    }

    #[staticmethod]
    fn october() -> Self {
        PyMonth { inner: Month::October }
    }

    #[staticmethod]
    fn november() -> Self {
        PyMonth { inner: Month::November }
    }

    #[staticmethod]
    fn december() -> Self {
        PyMonth { inner: Month::December }
    }

    fn number_from_month(&self) -> u32 {
        self.inner.number_from_month()
    }

    fn succ(&self) -> Self {
        PyMonth { inner: self.inner.succ() }
    }

    fn pred(&self) -> Self {
        PyMonth { inner: self.inner.pred() }
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.inner)
    }

    fn __repr__(&self) -> String {
        format!("Month.{:?}", self.inner)
    }

    fn __int__(&self) -> u32 {
        self.inner.number_from_month()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.inner != other.inner
    }

    fn __hash__(&self) -> u64 {
        self.inner.number_from_month() as u64
    }
}
