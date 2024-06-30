use crate::index;
use crate::objects;
use crate::objects::Object;
use anyhow::Context;

pub(crate) fn invoke(path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
    let index_path = ".git/index";
    let mut index = index::Index::init(index_path)?;
    let path = path.as_ref();
    if path.is_dir() {
        anyhow::bail!("add is not implemented for directories yet.");
    } else {
        let mut blob = objects::blob::Blob::from_file(path)
            .with_context(|| format!("Failed to create blob for: {}", path.display()))?;
        let oid = blob
            .write()
            .with_context(|| format!("Failed to write blob for: {}", path.display()))?;
        let meta = std::fs::metadata(path)
            .with_context(|| format!("Failed to get metadata for: {}", path.display()))?;
        let entry = index::entry::IndexEntry::new(path.to_path_buf(), oid, &meta);
        index.add(entry);
    }

    index
        .write(index_path)
        .with_context(|| format!("Failed to write index to: {}", index_path))
}
