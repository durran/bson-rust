/// The BSON enum.
use document::Document;

/// The enum for all valid BSON types.
#[derive(Clone, Debug, PartialEq)]
pub enum Bson {
    String(String),
    Document(Document)
}

#[macro_export]
macro_rules! bson {
    ($value:expr) => ($crate::Bson::String($value.to_string()));
}
