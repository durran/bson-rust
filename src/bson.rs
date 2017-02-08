/// The BSON enum.
use document::Document;

/// The enum for all valid BSON types.
#[derive(Clone, Debug, PartialEq)]
pub enum Bson {
    Double(f64), // 0x01
    String(String), // 0x02
    Document(Document), // 0x03
    Array(Vec<Bson>), // 0x04
    Undefined, // 0x06
    Boolean(bool), // 0x08
    Null, // 0x0A
    RegExp(String, String), // 0x0B
    Int32(i32), // 0x10
    Int64(i64) // 0x12
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

/// The from implementation for converting a `bool` to a `Bson::Boolean`.
impl From<bool> for Bson {

    /// Convert from a `bool` to a `Bson::Double`.
    ///
    /// # Parameters
    /// - `value` - The `bool` to convert from.
    ///
    /// # Returns
    /// The `Bson::Boolean`.
    fn from(value: bool) -> Bson {
        Bson::Boolean(value)
    }
}

/// The from implementation for converting a `i8` to a `Bson::Int32`.
impl From<i8> for Bson {

    /// Convert from a `i8` to a `Bson::Int32`.
    ///
    /// # Parameters
    /// - `value` - The `i8` to convert from.
    ///
    /// # Returns
    /// The `Bson::Int32`.
    fn from(value: i8) -> Bson {
        Bson::Int32(value as i32)
    }
}

/// The from implementation for converting a `i16` to a `Bson::Int32`.
impl From<i16> for Bson {

    /// Convert from a `i16` to a `Bson::Int32`.
    ///
    /// # Parameters
    /// - `value` - The `i16` to convert from.
    ///
    /// # Returns
    /// The `Bson::Int32`.
    fn from(value: i16) -> Bson {
        Bson::Int32(value as i32)
    }
}

/// The from implementation for converting a `i32` to a `Bson::Int32`.
impl From<i32> for Bson {

    /// Convert from a `i32` to a `Bson::Int32`.
    ///
    /// # Parameters
    /// - `value` - The `i32` to convert from.
    ///
    /// # Returns
    /// The `Bson::Int32`.
    fn from(value: i32) -> Bson {
        Bson::Int32(value)
    }
}

/// The from implementation for converting a `i64` to a `Bson::Int64`.
impl From<i64> for Bson {

    /// Convert from a `i64` to a `Bson::Int64`.
    ///
    /// # Parameters
    /// - `value` - The `i64` to convert from.
    ///
    /// # Returns
    /// The `Bson::Int64`.
    fn from(value: i64) -> Bson {
        Bson::Int64(value)
    }
}

/// Converts expressions in the macro to normal `Bson` variants.
#[macro_export]
macro_rules! bson {
    // Create an empty array when no values given.
    ([]) => ($crate::Bson::Array(Vec::new()));

    // Create an empty document when no values given.
    ({}) => ($crate::Bson::Document($crate::Document::new()));

    // When a value is provided convert it to the BSON type.
    ($value:expr) => (::std::convert::From::from($value));
}
