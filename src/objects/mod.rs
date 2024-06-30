/// This module contains the implementation of various objects used in the repository.
pub mod blob;
pub mod commit;
pub mod id;
pub mod kind;
pub mod tree;

use crate::objects;
use anyhow::Context;
use sha1::Digest;
use std::io::prelude::*;

/// The `Object` trait represents a generic object in the repository.
pub(crate) trait Object {
    /// Returns the kind of the object.
    fn kind(&self) -> &objects::kind::ObjectKind;

    /// Returns the size of the object in bytes.
    fn size(&self) -> u64;

    // Returns the contents of the object as a reader.
    fn content(&mut self) -> &mut dyn Read;

    /// Writes the object into the provided writer and returns the resulting object ID.
    ///
    /// # Arguments
    ///
    /// * `writer` - The writer to write the object into.
    ///
    /// # Returns
    ///
    /// The resulting object ID.
    ///
    /// # Errors
    ///
    /// Returns an `anyhow::Error` if there was an error writing the object into the writer.
    fn write_into(&mut self, writer: impl Write) -> anyhow::Result<objects::id::ObjectID> {
        let writer = flate2::write::ZlibEncoder::new(writer, flate2::Compression::default());
        let mut writer = HashWriter::new(writer);
        // Write the header of the object: 'kind' 'size in bytes''null-byte'
        write!(writer, "{} {}\0", &self.kind(), &self.size())?;
        // Copy the contents of the object (reader) into the writer
        let n = std::io::copy(self.content(), &mut writer).context("Stream blob into writer.")?;
        anyhow::ensure!(
            n == self.size(),
            ".git/object file did not have the expected size. Expected size: {}. Actual size: {n})",
            self.size()
        );

        let _ = writer.writer.finish()?;
        let hash = writer.hasher.finalize();
        Ok(objects::id::ObjectID::from_bytes(hash.into()))
    }

    /// Calculates the hash of the object and returns the resulting `ObjectID`.
    ///
    /// This method writes the object to a sink, compressing it with zlib and calculating its hash.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `ObjectID` of the written object if successful, or an
    /// `anyhow::Error` if an error occurs.
    fn hash(&mut self) -> anyhow::Result<objects::id::ObjectID> {
        self.write_into(std::io::sink())
    }

    /// Writes the object to the database.
    ///
    /// This method writes the object to a temporary file in the `.git/objects` directory,
    /// calculates its hash, and then moves it to the final object path.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `ObjectID` of the written object if successful, or an
    /// `anyhow::Error` if an error occurs.
    fn write(&mut self) -> anyhow::Result<objects::id::ObjectID> {
        let db_path = ".git/objects";
        let temp_filename = uuid::Uuid::new_v4().to_string();
        // Create the temporary path
        let temp_path = format!("{}/{}", &db_path, &temp_filename);
        let temp_path = std::path::Path::new(&temp_path);
        // Write the object in a file at the temporary path
        let file = std::fs::File::create(temp_path).context("Writing object in temporary file.")?;
        let object_id = self.write_into(file)?;
        let hash = object_id.to_string();
        // Create the final object path
        let object_path = format!("{}/{}/{}", &db_path, &hash[..2], &hash[2..]);
        let object_path = std::path::Path::new(&object_path);
        let _ = std::fs::create_dir_all(object_path.parent().unwrap())?;
        let _ = std::fs::rename(&temp_path, &object_path);
        Ok(object_id)
    }
}

/// Reads an object from the database given its hash.
///
/// # Arguments
///
/// * `hash` - The hash of the object.
///
/// # Returns
///
/// A tuple containing the object kind, size in bytes, and a buffered reader for reading the object's data.
///
/// # Errors
///
/// This function can return an error if there are any issues with reading the object from the database.
pub(crate) fn read_object(
    hash: &str,
) -> anyhow::Result<(objects::kind::ObjectKind, u64, impl BufRead)> {
    // Create the object path from its hash
    let path = format!(".git/objects/{}/{}", &hash[..2], &hash[2..]);

    // Read the file into a buffer: Read & decompress
    let file = std::fs::File::open(&path).context("Loading raw file from the database.")?;
    let reader = flate2::read::ZlibDecoder::new(file);
    let mut reader = std::io::BufReader::new(reader);
    let mut buffer = Vec::new();

    // Read header from the buffer: 'kind' 'size in bytes''null-byte'
    reader
        .read_until(0, &mut buffer)
        .context("Read header terminated by null byte.")?;
    let header = std::ffi::CStr::from_bytes_with_nul(&buffer)
        .context("We know there is one null byte at the end.")?
        .to_str()
        .context("The header is not properly UTF-8 encoded.")?;

    let (object_type, size) = match header.split_once(' ') {
        Some((object_type, size)) => (object_type, size),
        None => {
            anyhow::bail!(".git/objects file header did not start with a known type: '{header}'")
        }
    };

    // Parse the expected size of the object
    let size = size.parse::<u64>().context(format!(
        "Object header does not contain a valid size in bytes: {}",
        size
    ))?;

    // Take the expected number of bytes from the reader
    let reader = reader; //.take(size);

    // Return the object depending on its type
    let object_type = match object_type {
        "blob" => objects::kind::ObjectKind::Blob,
        "tree" => objects::kind::ObjectKind::Tree,
        "commit" => objects::kind::ObjectKind::Commit,
        _ => anyhow::bail!("Object kind is not one of the acceptables."),
    };

    Ok((object_type, size, reader))
}

/// A writer that calculates the SHA-1 hash of the written data.
pub(crate) struct HashWriter<W> {
    pub(crate) writer: W,
    pub(crate) hasher: sha1::Sha1,
}

impl<W: Write> HashWriter<W> {
    /// Constructs a new HashWriter.
    pub(crate) fn new(writer: W) -> Self {
        HashWriter {
            writer,
            hasher: sha1::Sha1::new(),
        }
    }
}

impl<W> Write for HashWriter<W>
where
    W: Write,
{
    /// Writes the given buffer to the writer and updates the hasher with the written data.
    ///
    /// # Arguments
    ///
    /// * `buf` - The buffer containing the data to be written.
    ///
    /// # Returns
    ///
    /// Returns the number of bytes written or an `std::io::Error` if an error occurred.
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = self.writer.write(buf)?;
        self.hasher.update(&buf[..n]);
        Ok(n)
    }

    /// Flushes the writer.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the flush operation was successful or an `std::io::Error` if an error occurred.
    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()?;
        Ok(())
    }
}
