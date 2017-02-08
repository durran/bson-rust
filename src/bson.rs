/// The BSON enum.
use document::Document;

/// The enum for all valid BSON types.
#[derive(Clone, Debug, PartialEq)]
pub enum Bson {
    Double(f64),
    String(String),
    Document(Document)
}

/// The from implementation for converting a `f64` to a `Bson::Double`.
impl From<f64> for Bson {

    /// Convert from a `f64` to a `Bson::Double`.
    ///
    /// # Parameters
    /// - `value` - The `f64` to convert from.
    ///
    /// # Returns
    /// The `Bson::Double`.
    fn from(value: f64) -> Bson {
        Bson::Double(value)
    }
}

/// The from implementation for converting a `&str` to a `Bson::String`.
impl<'a> From<&'a str> for Bson {

    /// Convert from a `&str` to a `Bson::String`.
    ///
    /// # Parameters
    /// - `value` - The `&str` to convert from.
    ///
    /// # Returns
    /// The `Bson::String`.
    fn from(value: &str) -> Bson {
        Bson::String(value.to_string())
    }
}

/// Converts expressions in the macro to normal `Bson` variants.
#[macro_export]
macro_rules! bson {
    ($value:expr) => (::std::convert::From::from($value));
}
