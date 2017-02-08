/// The BSON enum.
use document::Document;

/// The enum for all valid BSON types.
#[derive(Clone, Debug, PartialEq)]
pub enum Bson {
    String(String),
    Document(Document)
}
