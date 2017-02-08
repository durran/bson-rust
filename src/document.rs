use linked_hash_map::LinkedHashMap;
use bson::Bson;

/// Represents a BSON document.
#[derive(Clone, Debug, PartialEq)]
pub struct Document {
    elements: LinkedHashMap<String, Bson>
}

/// The implementation for `Document`.
impl Document {

    /// Create a new `Document`.
    ///
    /// # Returns
    /// The new `Document` instance.
    pub fn new() -> Document {
        Document {
            elements: LinkedHashMap::new()
        }
    }

    /// Get a value from the document for the provided key.
    ///
    /// # Parameters
    /// - `key` - The `&str` key.
    ///
    /// # Returns
    /// The `Option` with the `Bson` value.
    pub fn get(&self, key: &str) -> Option<&Bson> {
        return self.elements.get(key);
    }

    /// Insert an element into the `Document`.
    ///
    /// # Parameters
    /// - `key` - The `String` for the key.
    /// - `value` - The `Bson` value.
    ///
    /// # Returns
    /// The inserted `Bson` value in an `Option`.
    pub fn insert(&mut self, key: String, value: Bson) -> Option<Bson> {
        return self.elements.insert(key, value);
    }
}

/// Provides a convenient way for creating documents.
#[macro_export]
macro_rules! document {
    // If there are no arguments simply return an empty document.
    () => ($crate::Document::new());

    // If arguments are provided construct the document with the key/value pairs.
    ($($key:expr => $value:tt),*) => {
        {
            let mut document = $crate::Document::new();
            $(document.insert($key.to_string(), bson!($value));)*
            document
        }
    };
}
