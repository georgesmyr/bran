use anyhow::Context;
use std::io::Read;
use std::path::Path;

use crate::objects::kind::ObjectKind;
use crate::objects::read_object;
use crate::objects::Object;

#[derive(Clone)]
pub(crate) struct Blob<R> {
    size: u64,
    content: R,
}

impl Blob<()> {
    /// Creates a new `Blob` object with the given size and reader.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the blob object.
    /// * `reader` - An implementation of the `Read` trait that provides the data for the blob.
    ///
    /// # Returns
    ///
    /// Returns a `Blob` object with the specified size and reader.
    pub(crate) fn new(size: u64, reader: impl Read) -> Blob<impl Read> {
        Blob {
            size,
            content: reader,
        }
    }

    /// Creates a new `Blob` object from a file.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice representing the path to the file.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `Blob` with a reader if the file is successfully opened,
    /// or an `anyhow::Error` if opening the file or reading its metadata fails.
    pub(crate) fn from_file(path: impl AsRef<Path>) -> anyhow::Result<Blob<impl Read>> {
        let path = path.as_ref();
        let file = std::fs::File::open(path).context("Failed to open file.")?;
        let metadata = file.metadata().context("Failed to read file metadata.")?;
        if metadata.is_dir() {
            anyhow::bail!("Path points to directory.");
        }
        let size = metadata.len();
        Ok(Blob::new(size, file))
    }

    /// Reads a blob object from the given hash and returns a `Blob` with a reader.
    ///
    /// # Arguments
    ///
    /// * `hash` - A string slice representing the hash of the object to read.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `Blob` with a reader if the object is a blob,
    /// or an `anyhow::Error` if the object is not a blob or if reading the object fails.
    pub(crate) fn from_hash(hash: &str) -> anyhow::Result<Blob<impl Read>> {
        let (kind, size, reader) = read_object(hash).context("Failed to read object.")?;
        match kind {
            ObjectKind::Blob => Ok(Blob::new(size, reader)),
            _ => anyhow::bail!("Object is not a blob."),
        }
    }
}

impl<R> Object for Blob<R>
where
    R: Read,
{
    /// Returns the kind of the object.
    fn kind(&self) -> &ObjectKind {
        &ObjectKind::Blob
    }

    /// Returns the size of the object in bytes.
    fn size(&self) -> u64 {
        self.size
    }

    /// Returns a reference to the reader for the object's data.
    fn content(&mut self) -> &mut dyn Read {
        &mut self.content
    }
}
