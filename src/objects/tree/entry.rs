use std::ffi::{OsStr, OsString};
use std::fmt;

use crate::objects::id::ObjectID;
use crate::objects::kind::ObjectKind;
use crate::objects::tree::mode::EntryMode;

/// Represents an entry in a tree object.
#[derive(Debug)]
pub(crate) struct TreeEntry {
    name: OsString,
    mode: EntryMode,
    oid: Option<ObjectID>,
}

/// Represents an entry in a tree object.
/// Represents an entry in a tree object.
///
/// Each `TreeEntry` contains information about a file or directory within a tree object.
/// It includes the mode, name, and ID of the associated object.
impl TreeEntry {
    /// Creates a new `TreeEntry` instance.
    ///
    /// # Arguments
    ///
    /// * `mode` - The mode of the entry.
    /// * `name` - The name of the entry.
    /// * `oid` - The ID of the object associated with the entry.
    ///
    /// # Returns
    ///
    /// A new `TreeEntry` instance.
    pub(crate) fn new(name: &OsString, mode: EntryMode, oid: Option<ObjectID>) -> TreeEntry {
        TreeEntry {
            name: name.clone(),
            mode,
            oid,
        }
    }

    /// Returns the path of the entry.
    pub(crate) fn name(&self) -> &OsStr {
        &self.name
    }

    /// Returns the mode of the entry.
    pub(crate) fn mode(&self) -> &EntryMode {
        &self.mode
    }

    /// Return the kind of the entry.
    pub(crate) fn kind(&self) -> &ObjectKind {
        match self.mode {
            EntryMode::Directory => &ObjectKind::Tree,
            _ => &ObjectKind::Blob,
        }
    }

    /// Returns the ID of the object associated with the entry.
    pub(crate) fn oid(&self) -> &Option<ObjectID> {
        &self.oid
    }
}

impl fmt::Display for TreeEntry {
    /// Formats the `TreeEntry` struct for debugging purposes.
    ///
    /// This method formats the `TreeEntry` struct by writing its mode, kind, oid, and name
    /// to the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a `Formatter` object.
    ///
    /// # Returns
    ///
    /// This method returns a `fmt::Result` indicating whether the formatting was successful.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} ", self.mode(), self.kind())?;
        // Write the oid if it is present.
        _ = match self.oid() {
            Some(oid) => write!(f, "{}", oid),
            None => Ok(()),
        };
        write!(f, "\t{}", self.name().to_string_lossy())
    }
}
