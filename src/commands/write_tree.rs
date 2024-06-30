use crate::objects::tree::Tree;
use anyhow::Context;
use std::path::Path;

pub(crate) fn invoke(path: &Path) -> anyhow::Result<()> {
    let oid = Tree::write_for_dir(path)?.context("Failed to write tree.")?;
    println!("{}", oid.hash());
    Ok(())
}
