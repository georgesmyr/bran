use anyhow::Context;
use anyhow;
use std::fs;
use std::path::Path;
use std::io::prelude::*;
use std::io::BufReader;
use uuid::Uuid;
use std::ffi::CStr;

use crate::hashwriter::HashWriter;
use crate::oid::ObjectID;
use crate::kind::ObjectKind;

use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::Digest;
use flate2::read::ZlibDecoder;


/// Represents an object.
pub(crate) struct Object<R> {
    pub(crate) kind: ObjectKind,
    pub(crate) size: u64,
    pub(crate) reader: R,
}

impl Object<()> {
    /// Creates a blob object from a file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file.
    ///
    /// # Returns
    ///
    /// Returns an `Object` containing the blob object.
    pub(crate) fn blob_from_file(path: impl AsRef<Path>) -> anyhow::Result<Object<impl Read>> {
        let path = path.as_ref();
        let stat = fs::metadata(path).with_context(|| format!("Stat: {}", path.display()))?;
        let file = fs::File::open(path).with_context(|| format!("File: {}", path.display()))?;

        Ok(Object {
            kind: ObjectKind::Blob,
            size: stat.len(),
            reader: file,
        })
    }

    /// Reads an object from the database.
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the object.
    ///
    /// # Returns
    ///
    /// Returns an `Object` containing the read object.
    pub(crate) fn read(hash: &str) -> anyhow::Result<Object<impl BufRead>> {
        // Create the object path from its hash
        let path = format!(".git/objects/{}/{}", &hash[..2], &hash[2..]);

        // Read the file into a buffer: Read & decompress
        let file = fs::File::open(&path).context("Loading raw file from the database.")?;
        let reader = ZlibDecoder::new(file);
        let mut reader = BufReader::new(reader);
        let mut buffer = Vec::new();

        // Read header from the buffer: 'kind' 'size in bytes''null-byte'
        reader
            .read_until(0, &mut buffer)
            .context("Read header terminated by null byte.")?;
        let header = CStr::from_bytes_with_nul(&buffer)
            .context("We know there is one null byte at the end.")?;
        let header = header.to_str().context("The header is not properly UTF-8 encoded.")?;

        let (kind, size) = match header.split_once(' ') {
            Some((kind, size)) => (kind, size),
            None => anyhow::bail!(
                ".git/objects file header did not start with a known type: '{header}'"
            ),
        };

        let kind = match kind {
            "blob" => ObjectKind::Blob,
            "tree" => ObjectKind::Tree,
            "commit" => ObjectKind::Commit,
            _ => anyhow::bail!("Object kind is not one of the acceptables."),
        };

        let size = size
            .parse::<u64>()
            .context(format!("Object header does not contain a valid size in bytes: {}", size))?;
        let reader = reader.take(size);
        Ok(Object {
            kind,
            size,
            reader,
        })
    }
}

impl<R> Object<R> 
where R: Read {

    /// Calculates the hash of the object and returns the resulting `ObjectID`.
    ///
    /// This method writes the object to the given writer, compressing it with zlib and calculating its hash.
    ///
    /// # Arguments
    ///
    /// * `writer` - The writer to which the object will be written.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `ObjectID` of the written object if successful, or an `anyhow::Error` if an error occurs.
    pub(crate) fn write_to(mut self, writer: impl Write) -> anyhow::Result<ObjectID> {
        
        let writer = ZlibEncoder::new(writer, Compression::default());
        let mut writer = HashWriter::new(writer);

        write!(writer, "{} {}\0", &self.kind, &self.size)?;
        std::io::copy(&mut self.reader, &mut writer).context("Stream file into blob.")?;
        let _ = writer.writer.finish()?;
        let hash = writer.hasher.finalize();
        Ok(ObjectID::new(hash.into()))
    }

    /// Calculates the hash of the object and returns the resulting `ObjectID`.
    ///
    /// This method writes the object to a sink, compressing it with zlib and calculating its hash.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `ObjectID` of the written object if successful, or an `anyhow::Error` if an error occurs.
    pub(crate) fn hash(self) -> anyhow::Result<ObjectID> {
        self.write_to(std::io::sink())
        
    }

    /// Writes the object to the database.
    ///
    /// This method writes the object to a temporary file in the `.git/objects` directory, calculates its hash, and then moves it to the final object path.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `ObjectID` of the written object if successful, or an `anyhow::Error` if an error occurs.
    pub(crate) fn write(self) -> anyhow::Result<ObjectID> {
        let db_path = ".git/objects";
        let temp_filename = Uuid::new_v4().to_string();
        // Create the temporary path
        let temp_path = format!("{}/{}", &db_path, &temp_filename);
        let temp_path = Path::new(&temp_path);
        // Write the object in a file at the temporary path
        let file = fs::File::create(temp_path).context("Writing object in temporary file.")?;
        let object_id = self.write_to(file)?;
        let hash = object_id.to_string();
        // Create the final object path
        let object_path = format!("{}/{}/{}", &db_path, &hash[..2], &hash[2..]);
        let object_path = Path::new(&object_path);
        let _ = fs::create_dir_all(object_path.parent().unwrap())?;
        let _ = fs::rename(&temp_path, &object_path);
        Ok(object_id)
    }

    // pub(crate) fn contents(&mut self) -> anyhow::Result<String> {
    //     let mut string = String::new();
    //     let n = self.reader.read_to_string(&mut string).context("Failed to read from read file.")?;
    //     anyhow::ensure!(
    //         n == self.size as usize,
    //         "Read {} bytes, expected {} bytes.",
    //         n, self.size
    //     );
    //     Ok(string)
    // }
}