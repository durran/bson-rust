use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{Result, Write};
use document::Document;

/// The `DocumentSerializer` object that can serialize documents.
pub struct DocumentSerializer<'a, W: ?Sized> where W: Write + 'a {
    writer: &'a mut W,
}

/// Implementation for the `DocumentSerializer` object.
impl<'a, W> DocumentSerializer<'a, W> where W: Write + 'a {

    /// Create the new `DocumentSerializer` object.
    ///
    /// # Parameters
    /// - `writer` - The writer to use.
    ///
    /// # Returns
    /// The new `DocumentSerializer` object.
    pub fn new(writer: &mut W) -> DocumentSerializer<W> {
        DocumentSerializer {
            writer: writer
        }
    }

    /// Serialize the provided document to raw BSON.
    ///
    /// # Parameters
    /// - `document` - The BSON document.
    ///
    /// # Returns
    /// The `Result` object.
    pub fn serialize(&mut self, document: &Document) -> Result<()> {
        Ok(())
    }
}
