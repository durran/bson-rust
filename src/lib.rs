extern crate byteorder;

use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{Result, Write};

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
