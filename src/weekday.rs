use pyo3::prelude::*;
use chrono::Weekday;

#[pyclass(name = "Weekday")]
#[derive(Clone, Copy)]
pub struct PyWeekday {
    pub inner: Weekday,
}

#[pymethods]
impl PyWeekday {
    #[new]
    fn new(day: u32) -> PyResult<Self> {
        let weekday = match day {
            0 => Weekday::Mon,
            1 => Weekday::Tue,
            2 => Weekday::Wed,
            3 => Weekday::Thu,
            4 => Weekday::Fri,
            5 => Weekday::Sat,
            6 => Weekday::Sun,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Day must be 0-6")),
        };
        Ok(PyWeekday { inner: weekday })
    }

    #[staticmethod]
    fn monday() -> Self {
        PyWeekday { inner: Weekday::Mon }
    }

    #[staticmethod]
    fn tuesday() -> Self {
        PyWeekday { inner: Weekday::Tue }
    }

    #[staticmethod]
    fn wednesday() -> Self {
        PyWeekday { inner: Weekday::Wed }
    }

    #[staticmethod]
    fn thursday() -> Self {
        PyWeekday { inner: Weekday::Thu }
    }

    #[staticmethod]
    fn friday() -> Self {
        PyWeekday { inner: Weekday::Fri }
    }

    #[staticmethod]
    fn saturday() -> Self {
        PyWeekday { inner: Weekday::Sat }
    }

    #[staticmethod]
    fn sunday() -> Self {
        PyWeekday { inner: Weekday::Sun }
    }

    fn number_from_monday(&self) -> u32 {
        self.inner.number_from_monday()
    }

    fn number_from_sunday(&self) -> u32 {
        self.inner.number_from_sunday()
    }

    fn num_days_from_monday(&self) -> u32 {
        self.inner.num_days_from_monday()
    }

    fn num_days_from_sunday(&self) -> u32 {
        self.inner.num_days_from_sunday()
    }

    fn succ(&self) -> Self {
        PyWeekday { inner: self.inner.succ() }
    }

    fn pred(&self) -> Self {
        PyWeekday { inner: self.inner.pred() }
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.inner)
    }

    fn __repr__(&self) -> String {
        format!("Weekday.{:?}", self.inner)
    }

    fn __int__(&self) -> u32 {
        self.inner.number_from_monday() - 1
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.inner != other.inner
    }

    fn __hash__(&self) -> u64 {
        self.inner.number_from_monday() as u64
    }
}
