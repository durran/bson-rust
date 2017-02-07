use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{Result, Write};
use bson::Bson;
use document::Document;

/// The `TyepSerializer` object that can serialize BSON types.
pub struct TypeSerializer<'a, W: ?Sized> where W: Write + 'a {
    writer: &'a mut W,
}

/// Implementation for the `TypeSerializer` object.
impl<'a, W> TypeSerializer<'a, W> where W: Write + 'a {

    /// Create the new `TypeSerializer` object.
    ///
    /// # Parameters
    /// - `writer` - The writer to use.
    ///
    /// # Returns
    /// The new `TypeSerializer` object.
    pub fn new(writer: &mut W) -> TypeSerializer<W> {
        TypeSerializer {
            writer: writer
        }
    }

    /// Serialize the provided bson value to raw BSON.
    ///
    /// # Parameters
    /// - `bson` - The Bson variant.
    ///
    /// # Returns
    /// The `Result` object.
    pub fn serialize(&mut self, bson: &Bson) -> Result<()> {
        match bson {
            &Bson::String(ref value) => self.serialize_string(value),
            &Bson::Document(ref value) => self.serialize_document(value)
        }
    }

    fn serialize_document(&mut self, value: &Document) -> Result<()> {
        Ok(())
    }

    fn serialize_string(&mut self, value: &str) -> Result<()> {
        self.writer.write_i32::<LittleEndian>(value.len() as i32 + 1);
        self.writer.write_all(value.as_bytes());
        self.writer.write_u8(0);
        Ok(())
    }
}
