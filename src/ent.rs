use crate::Result;

pub trait Ent {
    fn get(&self, key: &str) -> Result<&Self>;
    fn as_bool(&self) -> Result<bool>;
    fn as_i64(&self) -> Result<i64>;
    fn as_u64(&self) -> Result<u64>;
    fn as_f64(&self) -> Result<f64>;
    fn as_str(&self) -> Result<&str>;
}
