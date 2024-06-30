use crate::objects;
use crate::objects::Object;
use anyhow::Context;

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
    pub(crate) fn new(size: u64, reader: impl std::io::Read) -> Blob<impl std::io::Read> {
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
    pub(crate) fn from_file(
        path: impl AsRef<std::path::Path>,
    ) -> anyhow::Result<Blob<impl std::io::Read>> {
        let path = path.as_ref();
        let file = std::fs::File::open(path).context("Failed to open file.")?;
        let metadata = file.metadata().context("Failed to read file metadata.")?;
        if metadata.is_dir() {
            anyhow::bail!("Path points to directory.");
        }
        let size = metadata.len();
        Ok(Blob::new(size, file))
    }
}

impl<R> Object for Blob<R>
where
    R: std::io::Read,
{
    /// Returns the kind of the object.
    fn kind(&self) -> &objects::kind::ObjectKind {
        &objects::kind::ObjectKind::Blob
    }

    /// Returns the size of the object in bytes.
    fn size(&self) -> u64 {
        self.size
    }

    /// Returns a reference to the reader for the object's data.
    fn content(&mut self) -> &mut dyn std::io::Read {
        &mut self.content
    }
}
