/// The BSON enum.
use document::Document;

/// The enum for all valid BSON types.
pub enum Bson {
    String(String),
    Document(Document)
}
