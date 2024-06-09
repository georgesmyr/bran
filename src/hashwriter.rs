use sha1::Digest;
use sha1::Sha1;
use std::io::Write;

pub(crate) struct HashWriter<W> {
    pub(crate) writer: W,
    pub(crate) hasher: Sha1,
}

impl<W: Write> HashWriter<W> {
    /// Constructs a new HashWriter.
    pub(crate) fn new(writer: W) -> Self {
        HashWriter {
            writer,
            hasher: Sha1::new(),
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
