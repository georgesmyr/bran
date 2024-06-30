use anyhow::Context;
use std::ffi::CStr;
use std::ffi::OsString;
use std::io::BufRead;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;

pub(crate) mod entry;
pub(crate) mod mode;

use crate::cmp::compare_base_name;
use crate::objects::blob::Blob;
use crate::objects::id::ObjectID;
use crate::objects::kind::ObjectKind;
use crate::objects::read_object;
use crate::objects::tree::entry::TreeEntry;
use crate::objects::tree::mode::EntryMode;
use crate::objects::Object;

#[allow(dead_code)]
pub(crate) struct Tree<R> {
    size: u64,
    reader: R,
}

impl Tree<()> {
    /// Creates a new `Tree` object with the specified `kind`, `size`, and `reader`.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the `Tree` object.
    /// * `reader` - The reader used to read the content of the `Tree` object.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the newly created `Tree` object, or an `anyhow::Error` if an error occurred.
    pub(crate) fn new(size: u64, reader: impl Read) -> Tree<impl Read> {
        Tree { size, reader }
    }

    pub(crate) fn write_for_dir(path: impl AsRef<Path>) -> anyhow::Result<Option<ObjectID>> {
        // Read the directory
        let path = path.as_ref();
        let mut dir = std::fs::read_dir(path)
            .with_context(|| format!("Failed to read directory: {}", path.display()))?;

        // Read tree entries
        let mut tree_entries = Vec::new();
        while let Some(direntry) = dir.next() {
            let direntry = direntry.with_context(|| format!("Bad entry in {}", path.display()))?;
            let entry_path = direntry.path();
            let filename = direntry.file_name();
            let metadata = direntry
                .metadata()
                .with_context(|| format!("Failed to read metadata for {}", entry_path.display()))?;
            // Ignore entries here
            if direntry.file_name() == ".git" {
                continue;
            }

            // Create a new tree entry with specified name and mode, but no object ID.
            tree_entries.push((entry_path, filename, EntryMode::from_metadata(&metadata)));
        }

        // Sort entries by base name.
        tree_entries.sort_unstable_by(|entry1, entry2| {
            compare_base_name(&entry1.1, &entry1.2, &entry2.1, &entry2.2)
        });

        let mut tree_object = Vec::new();
        for (entry_path, name, mode) in &tree_entries {
            let oid = match mode {
                EntryMode::Directory => {
                    // If the entry is a directory, get the hash recursively.
                    let Some(oid) = Tree::write_for_dir(entry_path)
                        .context(format!("Failed to write tree {}", entry_path.display()))?
                    else {
                        // If the directory is empty, skip it.
                        continue;
                    };
                    oid
                }
                _ => {
                    // If the entry is not a directory, it will be a blob.
                    let mut blob = Blob::from_file(entry_path).context(format!(
                        "Failed to create blob from file: {}",
                        entry_path.display()
                    ))?;
                    blob.write().context("Failed to write blob in database.")?
                }
            };
            // Write entry
            tree_object.extend(format!("{}", mode).as_bytes());
            tree_object.push(b' ');
            tree_object.extend(name.as_encoded_bytes());
            tree_object.push(0);
            tree_object.extend(oid.to_bytes());
        }

        if tree_object.is_empty() {
            Ok(None)
        } else {
            Ok(Some(
                Tree::new(tree_object.len() as u64, Cursor::new(tree_object))
                    .write()
                    .context("Failed to write tree in database.")?,
            ))
        }
    }

    /// Creates a `Tree` object from a given hash.
    ///
    /// # Arguments
    ///
    /// * `hash` - A string representing the hash of the tree object.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `Tree` object if successful, or an `anyhow::Error` if an error occurs.
    pub(crate) fn from_hash(hash: &str) -> anyhow::Result<Vec<TreeEntry>> {
        // Read object
        let (kind, _, mut reader) = read_object(hash)?;
        // If the object is not a tree, return an error
        match kind {
            ObjectKind::Tree => {}
            _ => anyhow::bail!("Object is not a tree."),
        }

        let mut entries = Vec::new();
        let mut buf = Vec::new();
        let mut hashbuf: [u8; 20] = [0; 20];
        loop {
            buf.clear();
            // Read until the next null byte. This contains the mode and type of the entry.
            let n = reader
                .read_until(0, &mut buf)
                .context("Failed to read object")?;
            // If there are no more bytes to read, break the loop.
            if n == 0 {
                break;
            }

            let mode_name = CStr::from_bytes_with_nul(&buf)
                .context("Failed convert mode and name to CStr.")?
                .to_str()
                .context("Failed to convert mode and name to str.")?;
            let (mode, name) = mode_name
                .split_once(' ')
                .context("Failed to split mode and name.")?;

            // Parse the tree entry mode. If it fails, bail.
            let mode = if let Some(mode) = EntryMode::from_str(mode) {
                mode
            } else {
                anyhow::bail!("Invalid tree entry mode.")
            };

            // Read the next 20 bytes. This is the hash of the entry.
            reader
                .read_exact(&mut hashbuf)
                .context("Failed to read object")?;

            let hash = ObjectID::from_bytes(hashbuf);
            let entry = TreeEntry::new(&OsString::from(name), mode, Some(hash));
            entries.push(entry);
        }

        Ok(entries)
    }
}

impl<R> Object for Tree<R>
where
    R: Read,
{
    fn kind(&self) -> &ObjectKind {
        &ObjectKind::Tree
    }

    fn size(&self) -> u64 {
        self.size
    }

    fn content(&mut self) -> &mut dyn Read {
        &mut self.reader
    }
}
