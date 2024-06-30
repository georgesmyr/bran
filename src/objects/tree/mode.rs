use std::fmt;
use std::fs::Metadata;
use std::os::unix::fs::PermissionsExt;

/// Represents the mode of an entry in a tree object.
#[derive(Debug)]
pub(crate) enum EntryMode {
    Directory,
    Symlink,
    Executable,
    NonExecutable,
}

impl EntryMode {
    /// Creates an `EntryMode` from the given metadata.
    ///
    /// # Arguments
    ///
    /// * `metadata` - The metadata of the entry.
    ///
    /// # Returns
    ///
    /// The corresponding `EntryMode` for the given metadata.
    pub(crate) fn from_metadata(metadata: &Metadata) -> Self {
        if metadata.is_dir() {
            EntryMode::Directory
        } else if metadata.is_symlink() {
            EntryMode::Symlink
        } else if metadata.permissions().mode() & 0o111 != 0 {
            EntryMode::Executable
        } else {
            EntryMode::NonExecutable
        }
    }

    /// Creates an `EntryMode` from the given string representation.
    ///
    /// # Arguments
    ///
    /// * `mode` - The string representation of the mode.
    ///
    /// # Returns
    ///
    /// The corresponding `EntryMode` for the given string representation, or an error if the mode is invalid.
    pub(crate) fn from_str(mode: &str) -> Option<Self> {
        match mode {
            "40000" | "040000" => Some(EntryMode::Directory),
            "120000" => Some(EntryMode::Symlink),
            "100755" => Some(EntryMode::Executable),
            "100644" => Some(EntryMode::NonExecutable),
            _ => None,
        }
    }

    /// Creates an `EntryMode` from the given octal mode representation.
    ///
    /// # Arguments
    ///
    /// * `mode` - The octal representation of the mode as an integer.
    ///
    /// # Returns
    ///
    /// The corresponding `EntryMode` for the given octal representation, or `None` if the mode is invalid.
    pub(crate) fn from_octal(mode: u32) -> Option<Self> {
        match mode {
            0o40000 => Some(EntryMode::Directory),
            0o120000 => Some(EntryMode::Symlink),
            0o100755 => Some(EntryMode::Executable),
            0o100644 => Some(EntryMode::NonExecutable),
            _ => None,
        }
    }
}

impl fmt::Display for EntryMode {
    /// Formats the `EntryMode` enum for debugging purposes.
    ///
    /// This method formats the `EntryMode` enum by writing its string representation to the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a `Formatter` object.
    ///
    /// # Returns
    ///
    /// This method returns a `fmt::Result` indicating whether the formatting was successful.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EntryMode::Directory => write!(f, "40000"),
            EntryMode::Symlink => write!(f, "120000"),
            EntryMode::Executable => write!(f, "100755"),
            EntryMode::NonExecutable => write!(f, "100644"),
        }
    }
}
