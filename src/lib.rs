//! Abstract object entities to support a common interface across
//! JSON, Python, and BSON

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    KeyNotFound,
    DataTypeMismatch,
}

mod ent;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "python")]
mod py;


pub use {
    self::ent::Ent
};