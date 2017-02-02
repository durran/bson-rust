extern crate byteorder;

use std::io::{Read, Result, Write};

mod string;

/// All types that can be converted to BSON must implement the
/// `Serializable` trait.
pub trait Serializable {

    /// Serializes the object to BSON. Implementation should write the raw
    /// bytes to the provided writer.
    ///
    /// # Parameters
    /// - `writer` - The `Writer` to write to.
    ///
    /// # Returns
    /// - `result` - The `Result` object.
    fn to_bson<W: Write + ?Sized>(&self, writer: &mut W) -> Result<()>;
}

/// All types that can be instantiated from BSON must implement the
/// `Deserializable` trait.
pub trait Deserializable<T> {

    /// Deserializes the BSON into an object, reading the bytes from the
    /// reader.
    ///
    /// # Parameters
    /// - `reader` - The `Reader` to read from.
    ///
    /// # Returns
    /// - `Result<T>` - The `Result` object with the value if ok.
    fn from_bson<R: Read + ?Sized>(reader: &mut R) -> Result<T>;
}
