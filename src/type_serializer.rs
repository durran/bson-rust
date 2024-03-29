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
            &Bson::Double(value) => self.serialize_double(value),
            &Bson::String(ref value) => self.serialize_string(value),
            &Bson::Document(ref value) => self.serialize_document(value),
            &Bson::Array(ref value) => self.serialize_array(value),
            &Bson::Binary(t, ref value) => self.serialize_binary(t, value),
            &Bson::Undefined => self.serialize_null(),
            &Bson::Boolean(value) => self.serialize_boolean(value),
            &Bson::DateTime(value) => self.serialize_datetime(value),
            &Bson::Null => self.serialize_null(),
            &Bson::RegExp(ref pattern, ref options) => self.serialize_regexp(pattern, options),
            &Bson::DbPointer(ref name, ref id) => self.serialize_dbpointer(name, id),
            &Bson::Code(ref code, ref scope) => self.serialize_code(code, scope),
            &Bson::Symbol(ref value) => self.serialize_string(value),
            &Bson::Int32(value) => self.serialize_i32(value),
            &Bson::Timestamp(value) => self.serialize_u64(value),
            &Bson::Int64(value) => self.serialize_i64(value),
            &Bson::MinKey => self.serialize_minkey(),
            &Bson::MaxKey => self.serialize_maxkey()
        }
    }

    fn serialize_double(&mut self, value: f64) -> Result<()> {
        Ok(())
    }

    fn serialize_string(&mut self, value: &str) -> Result<()> {
        self.writer.write_i32::<LittleEndian>(value.len() as i32 + 1);
        self.writer.write_all(value.as_bytes());
        self.writer.write_u8(0);
        Ok(())
    }

    fn serialize_document(&mut self, value: &Document) -> Result<()> {
        Ok(())
    }

    fn serialize_array(&mut self, value: &Vec<Bson>) -> Result<()> {
        Ok(())
    }

    fn serialize_binary(&mut self, t: u8, value: &Vec<u8>) -> Result<()> {
        Ok(())
    }

    fn serialize_null(&mut self) -> Result<()> {
        Ok(())
    }

    fn serialize_boolean(&mut self, value: bool) -> Result<()> {
        Ok(())
    }

    fn serialize_datetime(&mut self, value: i64) -> Result<()> {
        Ok(())
    }

    fn serialize_regexp(&mut self, pattern: &str, options: &str) -> Result<()> {
        Ok(())
    }

    fn serialize_dbpointer(&mut self, name: &str, id: &[u8; 12]) -> Result<()> {
        Ok(())
    }

    fn serialize_code(&mut self, code: &str, scope: &Document) -> Result<()> {
        Ok(())
    }

    fn serialize_i32(&mut self, value: i32) -> Result<()> {
        Ok(())
    }

    fn serialize_u64(&mut self, value: u64) -> Result<()> {
        Ok(())
    }

    fn serialize_i64(&mut self, value: i64) -> Result<()> {
        Ok(())
    }

    fn serialize_minkey(&mut self) -> Result<()> {
        Ok(())
    }

    fn serialize_maxkey(&mut self) -> Result<()> {
        Ok(())
    }
}
