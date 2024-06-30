use crate::index;

pub(crate) fn invoke(path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
    let index = index::Index::init(path)?;
    for entry in index.entries() {
        println!("{}", entry.path().display());
    }
    Ok(())
}
