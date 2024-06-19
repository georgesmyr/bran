use std::fs::Metadata;
use std::os::unix::fs::PermissionsExt;


#[derive(Debug)]
pub(crate) enum EntryMode {
    Directory,
    Symlink,
    Executable,
    NonExecutable,
}

impl EntryMode {

    pub(crate) fn from(metadata: Metadata) -> Self {
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

    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            EntryMode::Directory => "040000",
            EntryMode::Symlink => "120000",
            EntryMode::Executable => "100755",
            EntryMode::NonExecutable => "100644",
        }
    }

}