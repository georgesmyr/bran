use crate::index;
use std::path::Path;

pub(crate) fn invoke(path: &Path) -> anyhow::Result<()> {
    let index = index::Index::init(path)?;
    for entry in index.entries() {
        println!("{}", entry.path().display());
    }
    Ok(())
}
