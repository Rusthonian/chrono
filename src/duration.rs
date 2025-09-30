use pyo3::prelude::*;
use chrono::Duration;

#[pyclass(name = "Duration")]
#[derive(Clone, Copy)]
pub struct PyDuration {
    pub inner: Duration,
}

#[pymethods]
impl PyDuration {
    #[new]
    #[pyo3(signature = (*, weeks=None, days=None, hours=None, minutes=None, seconds=None, milliseconds=None, microseconds=None, nanoseconds=None))]
    fn new(
        weeks: Option<i64>,
        days: Option<i64>,
        hours: Option<i64>,
        minutes: Option<i64>,
        seconds: Option<i64>,
        milliseconds: Option<i64>,
        microseconds: Option<i64>,
        nanoseconds: Option<i64>,
    ) -> PyResult<Self> {
        let mut duration = Duration::zero();
        
        if let Some(w) = weeks {
            duration = duration + Duration::weeks(w);
        }
        if let Some(d) = days {
            duration = duration + Duration::days(d);
        }
        if let Some(h) = hours {
            duration = duration + Duration::hours(h);
        }
        if let Some(m) = minutes {
            duration = duration + Duration::minutes(m);
        }
        if let Some(s) = seconds {
            duration = duration + Duration::seconds(s);
        }
        if let Some(ms) = milliseconds {
            duration = duration + Duration::milliseconds(ms);
        }
        if let Some(us) = microseconds {
            duration = duration + Duration::microseconds(us);
        }
        if let Some(ns) = nanoseconds {
            duration = duration + Duration::nanoseconds(ns);
        }

        Ok(PyDuration { inner: duration })
    }

    #[staticmethod]
    fn zero() -> Self {
        PyDuration { inner: Duration::zero() }
    }

    #[staticmethod]
    fn min_value() -> Self {
        PyDuration { inner: Duration::MIN }
    }

    #[staticmethod]
    fn max_value() -> Self {
        PyDuration { inner: Duration::MAX }
    }

    #[staticmethod]
    fn weeks(weeks: i64) -> Self {
        PyDuration { inner: Duration::weeks(weeks) }
    }

    #[staticmethod]
    fn days(days: i64) -> Self {
        PyDuration { inner: Duration::days(days) }
    }

    #[staticmethod]
    fn hours(hours: i64) -> Self {
        PyDuration { inner: Duration::hours(hours) }
    }

    #[staticmethod]
    fn minutes(minutes: i64) -> Self {
        PyDuration { inner: Duration::minutes(minutes) }
    }

    #[staticmethod]
    fn seconds(seconds: i64) -> Self {
        PyDuration { inner: Duration::seconds(seconds) }
    }

    #[staticmethod]
    fn milliseconds(milliseconds: i64) -> Self {
        PyDuration { inner: Duration::milliseconds(milliseconds) }
    }

    #[staticmethod]
    fn microseconds(microseconds: i64) -> Self {
        PyDuration { inner: Duration::microseconds(microseconds) }
    }

    #[staticmethod]
    fn nanoseconds(nanoseconds: i64) -> Self {
        PyDuration { inner: Duration::nanoseconds(nanoseconds) }
    }

    fn num_weeks(&self) -> i64 {
        self.inner.num_weeks()
    }

    fn num_days(&self) -> i64 {
        self.inner.num_days()
    }

    fn num_hours(&self) -> i64 {
        self.inner.num_hours()
    }

    fn num_minutes(&self) -> i64 {
        self.inner.num_minutes()
    }

    fn num_seconds(&self) -> i64 {
        self.inner.num_seconds()
    }

    fn num_milliseconds(&self) -> i64 {
        self.inner.num_milliseconds()
    }

    fn num_microseconds(&self) -> Option<i64> {
        self.inner.num_microseconds()
    }

    fn num_nanoseconds(&self) -> Option<i64> {
        self.inner.num_nanoseconds()
    }

    fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }

    fn abs(&self) -> Self {
        PyDuration { inner: self.inner.abs() }
    }

    fn to_std(&self) -> PyResult<std::time::Duration> {
        self.inner.to_std()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    fn __add__(&self, other: &Self) -> Self {
        PyDuration { inner: self.inner + other.inner }
    }

    fn __sub__(&self, other: &Self) -> Self {
        PyDuration { inner: self.inner - other.inner }
    }

    fn __mul__(&self, rhs: i32) -> Self {
        PyDuration { inner: self.inner * rhs }
    }

    fn __truediv__(&self, rhs: i32) -> Self {
        PyDuration { inner: self.inner / rhs }
    }

    fn __neg__(&self) -> Self {
        PyDuration { inner: -self.inner }
    }

    fn __abs__(&self) -> Self {
        self.abs()
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

    fn __str__(&self) -> String {
        format!("{:?}", self.inner)
    }

    fn __repr__(&self) -> String {
        format!("Duration(seconds={})", self.inner.num_seconds())
    }

    fn __hash__(&self) -> u64 {
        self.inner.num_nanoseconds().unwrap_or(0) as u64
    }
}
