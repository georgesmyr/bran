use crate::objects::blob::Blob;
use crate::objects::Object;
use anyhow::Context;

/// Invokes the `hash-object` subcommand.
///
/// # Arguments
///
/// * `path` - Path of the file to hash.
/// * `write` - If True, write the object to the object database.
///
/// # Returns
///
/// * Hash of the object.
pub(crate) fn invoke(path: &str, write: bool) -> anyhow::Result<()> {
    let mut blob = Blob::from_file(path).with_context(|| format!("Unable to hash {}.", path))?;
    let hash = if write {
        blob.write().context("Failed to write blob in database.")?
    } else {
        blob.hash().context("Failed to hash blob.")?
    };

    let hash = hash.to_string();
    println!("{}", hash);

    Ok(())
}
