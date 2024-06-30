use std::fmt;

/// Represents an Object ID.
#[derive(Clone)]
pub(crate) struct ObjectID {
    bytes: [u8; 20],
}

/// The `ObjectID` struct provides methods for creating, manipulating, and converting object IDs.
impl ObjectID {
    /// Creates an `ObjectID` from a hash string.
    ///
    /// # Arguments
    ///
    /// * `hash` - A string slice that represents the hash.
    ///
    /// # Returns
    ///
    /// An `ObjectID` created from the given hash.
    pub(crate) fn from_hash(hash: impl AsRef<str>) -> ObjectID {
        let mut bytes = [0; 20];
        hex::decode_to_slice(hash.as_ref(), &mut bytes).unwrap();
        ObjectID::from_bytes(bytes)
    }

    /// Creates an `ObjectID` from a byte array.
    ///
    /// # Arguments
    ///
    /// * `bytes` - An array of 20 bytes representing the object ID.
    ///
    /// # Returns
    ///
    /// An `ObjectID` created from the given byte array.
    pub(crate) fn from_bytes(bytes: [u8; 20]) -> ObjectID {
        ObjectID { bytes }
    }

    /// Returns the hash string representation of the `ObjectID`.
    ///
    /// # Returns
    ///
    /// A string that represents the hash of the `ObjectID`.
    pub(crate) fn hash(&self) -> String {
        hex::encode(&self.bytes)
    }

    /// Returns the byte array representation of the `ObjectID`.
    ///
    /// # Returns
    ///
    /// An array of 20 bytes representing the `ObjectID`.
    pub(crate) fn to_bytes(&self) -> [u8; 20] {
        self.bytes
    }
}

impl fmt::Display for ObjectID {
    /// Formats the `ObjectID` for display.
    ///
    /// This method writes the `ObjectID` to the given formatter using the `to_string` method.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write to.
    ///
    /// # Returns
    ///
    /// This method returns a `fmt::Result` indicating whether the operation succeeded or not.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.hash())
    }
}

impl fmt::Debug for ObjectID {
    /// Formats the `ObjectID` for debugging.
    ///
    /// This method writes the `ObjectID` to the given formatter using the `hash` method.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write to.
    ///
    /// # Returns
    ///
    /// This method returns a `fmt::Result` indicating whether the operation succeeded or not.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.hash())
    }
}
