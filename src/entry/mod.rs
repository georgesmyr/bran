pub(crate) mod mode;
pub(crate) mod cmp;

use anyhow::Context;
use crate::entry::mode::EntryMode;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::ffi::OsString;


pub(crate) struct TreeEntry {
    pub(crate) path: PathBuf,
    pub(crate) name: OsString,
    pub(crate) mode: EntryMode,
}


impl TreeEntry {
    pub(crate) fn from_direntry(direntry: DirEntry) -> anyhow::Result<Self> {
        let path = direntry.path();
        let name = direntry.file_name();
        let metadata = direntry.metadata()
                .with_context(|| format!("Failed to get metadata for: {}", direntry.path().display()))?;
        let mode = EntryMode::from(metadata);
        Ok(Self { path, name, mode })
    }
}

