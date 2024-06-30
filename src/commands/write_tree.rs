use crate::objects::tree::Tree;
use anyhow::Context;
use std::path::Path;

/// Writes a tree object from the given path to the database, and prints the resulting object ID.
///
/// # Arguments
///
/// * `path` - The path to the directory to write the tree object from.
///
/// # Returns
///
/// Returns a `Result` containing the resulting object ID, or an `anyhow::Error` if an error occurs.
pub(crate) fn invoke(path: &Path) -> anyhow::Result<()> {
    let oid = Tree::write_for_dir(path)?.context("Failed to write tree.")?;
    println!("{}", oid.hash());
    Ok(())
}
