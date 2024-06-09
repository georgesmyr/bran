/// Represents an Object ID.
pub(crate) struct ObjectID {
    bytes: [u8; 20],
}

/// Represents an Object ID.
impl ObjectID {
    /// Creates a new ObjectID with the given bytes.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The bytes representing the Object ID.
    ///
    /// # Returns
    ///
    /// A new ObjectID instance.
    pub(crate) fn new(bytes: [u8; 20]) -> ObjectID {
        ObjectID { bytes }
    }

    /// Returns the hexadecimal hash representation of the Object ID.
    pub(crate) fn to_string(&self) -> String {
        self.bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
}
