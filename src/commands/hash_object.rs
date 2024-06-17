use crate::object::Object;
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
    let object = Object::blob_from_file(path).context("Open blob input file")?;
    let hash = if write {
        object.write().context("Write blob in database.")?
    } else {
        object.hash().context("Hash blob.")?
    };

    let hash = hash.to_string();
    println!("{}", hash);

    Ok(())
}
