use crate::objects::id::ObjectID;
use anyhow::Context;
use byteorder::{BigEndian, ReadBytesExt};
use filetime::FileTime;
use std::fs::Metadata;
use std::io::prelude::*;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

/// Represents an entry in the index.
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct IndexEntry {
    pub(crate) ctime: u32,    // Creation time
    pub(crate) ctime_ns: u32, // Creation time nanoseconds
    pub(crate) mtime: u32,    // Modification time
    pub(crate) mtime_ns: u32, // Modification time nanoseconds
    pub(crate) dev: u32,      // Device ID
    pub(crate) ino: u32,      // Inode number
    pub(crate) mode: u32,     // File mode
    pub(crate) uid: u32,      // User ID
    pub(crate) gid: u32,      // Group ID
    pub(crate) size: u32,     // File size
    pub(crate) flags: u16,    // Flags
    pub(crate) oid: ObjectID, // Object ID
    pub(crate) path: PathBuf, // Path
}

impl IndexEntry {
    /// Parses an index entry from a reader.
    ///
    /// The reader is expected to be at the start of the index entry.
    /// First, we read 64 bytes from the file. This is the smallest size that an entry can be;
    /// ten 4-byte numbers, a 20-byte object ID, two bytes of flags and then a null-terminated string,
    /// which must be at least one character and a null byte, all of which adds up to 64 bytes.
    /// An entry must end with a null byte and its size must be a multiple of 8 bytes.
    ///
    /// Until the last byte of entry is \0, we read 8-byte blocks from the file and add them to entry.
    /// All numbers are stored in big-endian format.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a type that implements the `BufRead` trait.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the parsed `IndexEntry` if successful, or an `anyhow::Error` if parsing fails.
    pub(crate) fn parse<R: BufRead>(reader: &mut R) -> anyhow::Result<IndexEntry> {
        let ctime = reader.read_u32::<BigEndian>()?;
        let ctime_ns = reader.read_u32::<BigEndian>()?;
        let mtime = reader.read_u32::<BigEndian>()?;
        let mtime_ns = reader.read_u32::<BigEndian>()?;
        let dev = reader.read_u32::<BigEndian>()?;
        let ino = reader.read_u32::<BigEndian>()?;
        let mode = reader.read_u32::<BigEndian>()?;
        let uid = reader.read_u32::<BigEndian>()?;
        let gid = reader.read_u32::<BigEndian>()?;
        let size = reader.read_u32::<BigEndian>()?;
        let oid = {
            let mut oid_bytes = [0u8; 20];
            reader.read_exact(&mut oid_bytes)?;
            ObjectID::from_bytes(oid_bytes)
        };
        let flags = reader.read_u16::<BigEndian>()?;

        let mut path_buffer: Vec<u8> = Vec::new();
        let _ = reader
            .read_to_end(&mut path_buffer)
            .context("Failed to read path bytes to end")?;
        path_buffer.retain(|&x| x != 0);
        anyhow::ensure!(
            path_buffer.len() as u16 == flags,
            "Path length does not match flags."
        );

        let path = PathBuf::from(String::from_utf8(path_buffer).context("Failed to parse path.")?);

        Ok(IndexEntry {
            ctime,
            ctime_ns,
            mtime,
            mtime_ns,
            dev,
            ino,
            mode,
            uid,
            gid,
            size,
            flags,
            oid,
            path,
        })
    }

    /// Returns the path of the entry.
    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    /// Creates a new instance of the `IndexEntry` struct.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file.
    /// * `oid` - The object ID of the file.
    /// * `meta` - The metadata of the file.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the `IndexEntry` struct.
    pub(crate) fn new(path: PathBuf, oid: ObjectID, meta: &Metadata) -> Self {
        let ctime = FileTime::from_creation_time(meta).unwrap_or_else(|| FileTime::zero());
        let mtime = FileTime::from_last_modification_time(meta);
        let path_len = path.to_string_lossy().len();

        IndexEntry {
            path,
            oid,
            ctime: ctime.seconds() as u32,
            ctime_ns: ctime.nanoseconds() as u32,
            mtime: mtime.seconds() as u32,
            mtime_ns: mtime.nanoseconds() as u32,
            dev: meta.dev() as u32,
            ino: meta.ino() as u32,
            mode: meta.mode() as u32,
            uid: meta.uid() as u32,
            gid: meta.gid() as u32,
            size: meta.size() as u32,
            flags: path_len as u16,
        }
    }
}
