/// The BSON enum.
use document::Document;

/// The enum for all valid BSON types.
#[derive(Clone, Debug, PartialEq)]
pub enum Bson {
    Double(f64), // 0x01
    String(String), // 0x02
    Document(Document), // 0x03
    Array(Vec<Bson>), // 0x04
    Binary(u8, Vec<u8>), // 0x05
    Undefined, // 0x06
    Boolean(bool), // 0x08
    DateTime(i64), //0x09
    Null, // 0x0A
    RegExp(String, String), // 0x0B
    Code(String, Document), // 0x0D or 0F
    Symbol(String), //0x0E
    Int32(i32), // 0x10
    Timestamp(u64), // 0x11
    Int64(i64), // 0x12
    MinKey, // 0xFF
    MaxKey // 0x7F
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

/// The from implementation for converting a `u64` to a `Bson::Timestamp`.
impl From<u64> for Bson {

    /// Convert from a `u64` to a `Bson::Timestamp`.
    ///
    /// # Parameters
    /// - `value` - The `u64` to convert from.
    ///
    /// # Returns
    /// The `Bson::Timestamp`.
    fn from(value: u64) -> Bson {
        Bson::Timestamp(value)
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

    // Creating a BSON array with values present.
    ([$($value:tt),*]) => {{
        let mut vec = Vec::new();
        $(vec.push(bson!($value));)*
        $crate::Bson::Array(vec)
    }};

    // Create an empty document when no values given.
    ({}) => ($crate::Bson::Document($crate::Document::new()));

    // Create a BSON document from the values in key => value format.
    ({ $($key:expr => $value:tt),* }) => {
        {
            $crate::Bson::Document(document! {
                $($key => $value),*
            })
        }
    };

    // When a value is provided convert it to the BSON type.
    ($value:expr) => (::std::convert::From::from($value));
}

/// Convenience for defining BSON binary objects.
#[macro_export]
macro_rules! bson_binary {
    ($subtype:expr, $value:expr) => {
        {
            $crate::Bson::Binary($subtype, $value)
        }
    };
}

/// Convenience for defining BSON code objects.
#[macro_export]
macro_rules! bson_code {
    ($code:expr, $scope:expr) => {
        {
            $crate::Bson::Code($code.to_string(), $scope)
        }
    };
}

/// Convenience for defining BSON symbol objects.
#[macro_export]
macro_rules! bson_symbol {
    ($value:expr) => ($crate::Bson::Symbol($value.to_string()));
}

/// Convenience for defining BSON regexs.
#[macro_export]
macro_rules! bson_regexp {
    ($pattern:expr, $options:expr) => {
        {
            $crate::Bson::RegExp($pattern.to_string(), $options.to_string())
        }
    };
}

/// Convenience for defining BSON undefined objects.
#[macro_export]
macro_rules! bson_undefined {
    () => ($crate::Bson::Undefined);
}

/// Convenience for defining BSON null objects.
#[macro_export]
macro_rules! bson_null {
    () => ($crate::Bson::Null);
}

/// Convenience for defining BSON datetime objects.
#[macro_export]
macro_rules! bson_datetime {
    ($value:expr) => ($crate::Bson::DateTime($value));
}

/// Convenience for defining BSON min key objects.
#[macro_export]
macro_rules! bson_minkey {
    () => ($crate::Bson::MinKey);
}

/// Convenience for defining BSON max key objects.
#[macro_export]
macro_rules! bson_maxkey {
    () => ($crate::Bson::MaxKey);
}
