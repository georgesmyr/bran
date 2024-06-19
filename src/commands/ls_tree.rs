use crate::objects::kind::ObjectKind;
use crate::objects::Object;
use anyhow::Context;
use std::ffi::CStr;
use std::io::prelude::*;

pub(crate) fn invoke(hash: &str, name_only: bool) -> anyhow::Result<()> {
    let mut object = Object::read(hash).context("Failed to read object")?;

    match object.kind {
        ObjectKind::Tree => {
            let mut buf = Vec::new();
            let mut hashbuf: [u8; 20] = [0; 20];
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();

            loop {
                buf.clear();
                // Read until the next null byte. This contains the mode and type of the entry.
                let n = object
                    .reader
                    .read_until(0, &mut buf)
                    .context("Failed to read object")?;
                // If there are no more bytes to read, break the loop.
                if n == 0 {
                    break;
                }

                // Read the next 20 bytes. This is the hash of the entry.
                object
                    .reader
                    .read_exact(&mut hashbuf)
                    .context("Failed to read object")?;

                let mode_and_type =
                    CStr::from_bytes_with_nul(&buf).context("Failed to convert to CStr")?;
                let mut mode_and_type = mode_and_type.to_bytes().splitn(2, |&b| b == b' ');
                let mode = mode_and_type.next().context("Failed to get mode")?;
                let name = mode_and_type.next().context("Failed to get name")?;

                if name_only {
                    // If name_only is true, only print the name of the entry.
                    stdout
                        .write_all(name)
                        .context("Failed to write entry name to stdout")?;
                } else {
                    // If name_only is false, print the mode, type, and hash of the entry.
                    let mode =
                        std::str::from_utf8(mode).context("Failed to convert mode to str")?;
                    let hash = hex::encode(&hashbuf);
                    let object = Object::read(&hash)
                        .with_context(|| format!("Failed to read object with hash: {}", hash))?;
                    write!(stdout, "{mode:0>6} {} {}\t", object.kind, hash)
                        .context("Failed to write entry to stdout")?;
                    stdout.write_all(name).context("Failed to write entry name to stdout")?;
                }
                writeln!(stdout, "").context("Failed to write newline to stdout")?;
            }
        }
        _ => anyhow::bail!("Object is not a tree."),
    }

    Ok(())
}
