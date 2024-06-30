pub(crate) mod entry;

use crate::cmp::compare_base_name;
use crate::index::entry::IndexEntry;
use crate::objects::tree::mode::EntryMode;
use anyhow::Context;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{prelude::*, BufReader, Cursor};
use std::path::Path;

const INDEX_VERSIONS: [u32; 3] = [2, 3, 4];

/// Represents an index used for tracking changes in a Git repository.
pub(crate) struct Index<R> {
    version: u32,
    entries: Vec<IndexEntry>,
    reader: BufReader<R>,
}

impl Index<()> {
    /// Creates a new instance of the `Index` struct.
    ///
    /// # Arguments
    ///
    /// * `version` - The version of the index.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the newly created `Index` instance, or an `anyhow::Error`
    /// if an error occurs.
    pub(crate) fn init(path: impl AsRef<Path>) -> anyhow::Result<Index<File>> {
        let path = path.as_ref();
        let index_exists = path.exists();

        // Open index file with read/write and create permissions
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
            .with_context(|| format!("Failed to open index file: {}", path.display()))?;

        // Acquire exclusive lock on the file
        // file.lock_exclusive().with_context(|| {
        //     format!(
        //         "Failed to acquire exclusive lock for file: {}",
        //         path.display()
        //     )
        // })?;

        // If the index exists, parse it; otherwise, create one.
        if index_exists {
            // Parse the index file
            Index::parse_index(file)
        } else {
            // Create a new index file
            Ok(Index::new(2, Vec::new(), file))
        }
    }
}

impl<R: Read> Index<R> {
    /// Creates a new instance of the `Index` struct.
    ///
    ///  # Arguments
    ///
    ///  * `version` - The version of the index.
    ///  * `entries` - A vector of `IndexEntry` instances.
    ///  * `reader` - A type that implements the `Read` trait.
    ///  
    /// # Returns
    ///
    /// Returns a new instance of the `Index` struct.
    pub(crate) fn new(version: u32, entries: Vec<IndexEntry>, reader: R) -> Index<R> {
        Index {
            version,
            entries,
            reader: BufReader::new(reader),
        }
    }

    /// Parses an existing index file.
    ///
    /// # Arguments
    ///
    /// * `file` - The file to parse.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the parsed `Index` instance, or an `anyhow::Error`
    /// if an error occurs.
    fn parse_index(file: R) -> anyhow::Result<Index<R>> {
        let mut reader = BufReader::new(file);
        // let mut hasher = Sha1::new();

        // Parse signature from header and check its validity. The signature should be "DIRC".
        let mut buffer = [0u8; 4];
        reader
            .read_exact(&mut buffer)
            .context("Failed to read signature from header.")?;
        let signature = std::str::from_utf8(&buffer).context("Failed to parse index signature.")?;
        anyhow::ensure!(
            signature == "DIRC",
            format!("Invalid index signature: {}", signature)
        );
        // hasher.update(buffer);

        // Parse version from header and check its validity. The version should be 2, 3, or 4.
        let mut buffer = [0u8; 4];
        reader
            .read_exact(&mut buffer)
            .context("Failed to read index version")?;
        let version = u32::from_be_bytes(buffer);
        anyhow::ensure!(
            INDEX_VERSIONS.contains(&version),
            "Invalid index version: {}",
            version
        );
        // hasher.update(buffer);

        // Parse entry count from header.
        let mut buffer = [0u8; 4];
        reader
            .read_exact(&mut buffer)
            .context("Failed to read index entry count")?;
        let entry_count = u32::from_be_bytes(buffer);
        // hasher.update(buffer);

        // Parse entries.
        // For each entry read the 64 first bytes, which is the minimum possible entry length.
        // If the last byte is not nul, keep reading batches of 8 bytes until a nul byte is found.
        let mut entries = Vec::new();
        let mut entry: Vec<u8> = Vec::new();
        for _ in 0..entry_count {
            entry.clear();
            let mut buffer = [0u8; 64];
            reader
                .read_exact(&mut buffer)
                .context("Failed to read first 64 bytes of entry")?;
            entry.extend(&buffer);

            if entry.last().unwrap() != &0 {
                loop {
                    let mut buffer = [0u8; 8];
                    reader
                        .read_exact(&mut buffer)
                        .context("Failed to read for entry.")?;
                    entry.extend(&buffer);
                    if buffer[7] == 0 {
                        break;
                    }
                }
            }
            // hasher.update(&entry);
            let mut cursor = Cursor::new(&entry);
            entries.push(IndexEntry::parse(&mut cursor)?);
        }

        // Add hexsum validation

        Ok(Index {
            version,
            entries,
            reader,
        })
    }

    /// Returns a reference to the entries in the index.
    pub(crate) fn entries(&self) -> &Vec<IndexEntry> {
        &self.entries
    }

    /// Orders the entries in the index by path.
    fn sort_entries(&mut self) -> anyhow::Result<()> {
        self.entries.sort_unstable_by(|entry1, entry2| {
            compare_base_name(
                &entry1.path.as_os_str(),
                &EntryMode::from_octal(entry1.mode).unwrap(),
                &entry2.path.as_os_str(),
                &EntryMode::from_octal(entry2.mode).unwrap(),
            )
        });
        Ok(())
    }

    /// Adds an entry to the index. The index is updated in the following way. If an entry with the
    /// same name is already present in the index, it is replaced by the new entry. Naturally, there
    /// cannot be an file and a directory at the same time. Therefore, if we add a directory with
    /// the name of a file that is already present in the index, the file is removed. Similarly, if
    /// we add a file with the name of a directory that is already present in the index, the whole
    /// directory is removed.
    ///
    /// # Arguments
    ///
    /// * `entry` - The entry to be added to the index.
    pub(crate) fn add(&mut self, entry: IndexEntry) {
        self.entries.push(entry);
    }

    /// Writes the index to the file.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containint `()` if the index was written successfully, or an
    /// `anyhow::Error` if an error occurs.
    pub(crate) fn write(mut self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path.as_ref())?;

        // Write signature
        file.write_all(b"DIRC")?;
        // Write index version
        file.write_all(&self.version.to_be_bytes())?;
        // Write entry count
        file.write_all(&(self.entries.len() as u32).to_be_bytes())?;

        // Write entries
        self.sort_entries()?;
        let mut entry_buffer = Vec::new();
        for entry in &self.entries {
            entry_buffer.clear();
            entry_buffer.extend(&entry.ctime.to_be_bytes());
            entry_buffer.extend(&entry.ctime_ns.to_be_bytes());
            entry_buffer.extend(&entry.mtime.to_be_bytes());
            entry_buffer.extend(&entry.mtime_ns.to_be_bytes());
            entry_buffer.extend(&entry.dev.to_be_bytes());
            entry_buffer.extend(&entry.ino.to_be_bytes());
            entry_buffer.extend(&entry.mode.to_be_bytes());
            entry_buffer.extend(&entry.uid.to_be_bytes());
            entry_buffer.extend(&entry.gid.to_be_bytes());
            entry_buffer.extend(&entry.size.to_be_bytes());
            entry_buffer.extend(entry.oid.to_bytes());
            entry_buffer.extend(&entry.flags.to_be_bytes());
            entry_buffer.extend(entry.path.to_string_lossy().as_bytes());
            let padding = 8 - (entry_buffer.len() % 8);
            entry_buffer.extend(vec![0; padding]);
            file.write_all(&entry_buffer)
                .context("Failed to write entry in index")?;
        }

        Ok(())
    }
}
