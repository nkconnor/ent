use crate::{Ent, Error, Result};
use pyo3::conversion::FromPyObject;
use pyo3::types::PyDict;
use pyo3::PyObject;


impl Ent for PyObject {
    fn get(&self, key: &str) -> Result<&Self> {
        self.downcast::<PyDict>()
            .map_err(|_| Error::DataTypeMismatch)?
            .get_item(key)
            .map(Ok)
            .unwrap_or(Err(Error::DataTypeMismatch))
    }

    fn as_bool(&self) -> Result<bool> {
        FromPyObject::extract(self).map_err(|_| Error::DataTypeMismatch)
    }

    fn as_i64(&self) -> Result<i64> {
        FromPyObject::extract(self).map_err(|_| Error::DataTypeMismatch)
    }

    fn as_u64(&self) -> Result<u64> {
        FromPyObject::extract(self).map_err(|_| Error::DataTypeMismatch)
    }

    fn as_f64(&self) -> Result<f64> {
        FromPyObject::extract(self).map_err(|_| Error::DataTypeMismatch)
    }

    fn as_str(&self) -> Result<&str> {
        FromPyObject::extract(self).map_err(|_| Error::DataTypeMismatch)
    }
}