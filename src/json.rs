use crate::{Ent, Error, Result};

use serde_json::Value;

impl Ent for Value {
    fn get(&self, key: &str) -> Result<&Self> {
        self.get(key).map(Ok).unwrap_or(Err(Error::KeyNotFound))
    }

    fn as_bool(&self) -> Result<bool> {
        self.as_bool()
            .map(Ok)
            .unwrap_or(Err(Error::DataTypeMismatch))
    }

    fn as_i64(&self) -> Result<i64> {
        self.as_i64()
            .map(Ok)
            .unwrap_or(Err(Error::DataTypeMismatch))
    }

    fn as_u64(&self) -> Result<u64> {
        self.as_u64()
            .map(Ok)
            .unwrap_or(Err(Error::DataTypeMismatch))
    }

    fn as_f64(&self) -> Result<f64> {
        self.as_f64()
            .map(Ok)
            .unwrap_or(Err(Error::DataTypeMismatch))
    }

    fn as_str(&self) -> Result<&str> {
        self.as_str()
            .map(Ok)
            .unwrap_or(Err(Error::DataTypeMismatch))
    }
}