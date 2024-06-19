use std::fmt;

/// Represents the kind of object.
pub(crate) enum ObjectKind {
    Blob,
    Tree,
    Commit,
}

/// Implements the `Display` trait for `ObjectKind`.
/// This allows `ObjectKind` instances to be formatted as strings.
impl fmt::Display for ObjectKind {
    /// Formats the `ObjectKind` instance as a string.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a `Formatter` instance.
    ///
    /// # Returns
    ///
    /// Returns a `fmt::Result` indicating the success or failure of the formatting operation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectKind::Blob => write!(f, "blob"),
            ObjectKind::Tree => write!(f, "tree"),
            ObjectKind::Commit => write!(f, "commit"),
        }
    }
}
