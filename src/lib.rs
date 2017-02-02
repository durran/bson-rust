extern crate byteorder;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Result, Write};

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

/// Implements BSON serialization for `String` types.
impl Serializable for String {

    /// Serializes the object to BSON. Implementation should write the raw
    /// bytes to the provided writer.
    ///
    /// # Parameters
    /// - `writer` - The `Writer` to write to.
    ///
    /// # Returns
    /// - `result` - The `Result` object.
    fn to_bson<W: Write + ?Sized>(&self, writer: &mut W) -> Result<()> {
        writer.write_i32::<LittleEndian>(self.len() as i32 + 1);
        writer.write_all(self.as_bytes());
        writer.write_u8(0);
        Ok(())
    }
}

/// Implements BSON deserializtion for `String` types.
impl Deserializable<String> for String {

    /// Deserializes the BSON into an object, reading the bytes from the
    /// reader.
    ///
    /// # Parameters
    /// - `reader` - The `Reader` to read from.
    ///
    /// # Returns
    /// - `Result<String>` - The `Result` object with the string value if ok.
    fn from_bson<R: Read + ?Sized>(reader: &mut R) -> Result<String> {
        let length = reader.read_i32::<LittleEndian>().unwrap();
        let mut string = String::with_capacity(length as usize - 1);
        reader.take(length as u64 - 1).read_to_string(&mut string);
        reader.read_u8();
        Ok(string)
    }
}
