use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

pub mod datetime;
pub mod naive;
pub mod duration;
pub mod timezone;
pub mod format;
pub mod constants;
pub mod errors;
pub mod utils;
pub mod weekday;
pub mod month;

use crate::datetime::{PyDateTime, PyDateTimeUtc, PyDateTimeLocal, PyDateTimeFixed};
use crate::naive::{PyNaiveDateTime, PyNaiveDate, PyNaiveTime};
use crate::duration::PyDuration;
use crate::timezone::{PyFixedOffset, PyUtc, PyLocal};
use crate::format::{parse_datetime, parse_date, parse_time, parse_from_str, parse_from_rfc3339, parse_from_rfc2822};
use crate::constants::ChronoConstants;
use crate::utils::{now_utc, now_local, timestamp_millis, timestamp_nanos, timestamp_micros};
use crate::weekday::PyWeekday;
use crate::month::PyMonth;

#[pymodule]
fn rusthonian_chrono(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDateTime>()?;
    m.add_class::<PyDateTimeUtc>()?;
    m.add_class::<PyDateTimeLocal>()?;
    m.add_class::<PyDateTimeFixed>()?;
    m.add_class::<PyNaiveDateTime>()?;
    m.add_class::<PyNaiveDate>()?;
    m.add_class::<PyNaiveTime>()?;
    m.add_class::<PyDuration>()?;
    m.add_class::<PyFixedOffset>()?;
    m.add_class::<PyUtc>()?;
    m.add_class::<PyLocal>()?;
    m.add_class::<PyWeekday>()?;
    m.add_class::<PyMonth>()?;

    m.add_function(wrap_pyfunction!(now_utc, m)?)?;
    m.add_function(wrap_pyfunction!(now_local, m)?)?;
    m.add_function(wrap_pyfunction!(timestamp_millis, m)?)?;
    m.add_function(wrap_pyfunction!(timestamp_nanos, m)?)?;
    m.add_function(wrap_pyfunction!(timestamp_micros, m)?)?;
    
    m.add_function(wrap_pyfunction!(parse_datetime, m)?)?;
    m.add_function(wrap_pyfunction!(parse_date, m)?)?;
    m.add_function(wrap_pyfunction!(parse_time, m)?)?;
    m.add_function(wrap_pyfunction!(parse_from_str, m)?)?;
    m.add_function(wrap_pyfunction!(parse_from_rfc3339, m)?)?;
    m.add_function(wrap_pyfunction!(parse_from_rfc2822, m)?)?;

    let py = m.py();
    ChronoConstants::register_constants(py, m)?;

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__doc__", "Complete Python bindings for the Rust chrono crate")?;

    Ok(())
}

pub fn setup_chrono_module(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    rusthonian_chrono(m)
}
