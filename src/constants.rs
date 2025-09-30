use pyo3::prelude::*;
use chrono::Datelike;

pub struct ChronoConstants;

impl ChronoConstants {
    pub fn register_constants(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add("MIN_YEAR", chrono::NaiveDate::MIN.year())?;
        m.add("MAX_YEAR", chrono::NaiveDate::MAX.year())?;
        
        m.add("MONDAY", 0)?;
        m.add("TUESDAY", 1)?;
        m.add("WEDNESDAY", 2)?;
        m.add("THURSDAY", 3)?;
        m.add("FRIDAY", 4)?;
        m.add("SATURDAY", 5)?;
        m.add("SUNDAY", 6)?;
        
        m.add("JANUARY", 1)?;
        m.add("FEBRUARY", 2)?;
        m.add("MARCH", 3)?;
        m.add("APRIL", 4)?;
        m.add("MAY", 5)?;
        m.add("JUNE", 6)?;
        m.add("JULY", 7)?;
        m.add("AUGUST", 8)?;
        m.add("SEPTEMBER", 9)?;
        m.add("OCTOBER", 10)?;
        m.add("NOVEMBER", 11)?;
        m.add("DECEMBER", 12)?;
        
        Ok(())
    }
}
