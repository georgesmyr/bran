// use anyhow::Context;
// use std::path::Path;
// use crate::entry::mode::EntryMode;
// use crate::entry::TreeEntry;
// use crate::entry::cmp::compare_base_name;
// use crate::object::Object;

// fn write_tree_for(path: &Path) -> anyhow::Result<[u8;20]> {

//     let mut dir = std::fs::read_dir(path)
//             .with_context(|| format!("Failed to read directory: {}", path.display()))?;

//     let mut tree_entries = Vec::new();
//     while let Some(entry) = dir.next() {
//         let direntry = entry.with_context(|| format!("Bad entry in {}", path.display()))?;
//         let tree_entry = TreeEntry::from_direntry(direntry).context("Failed to create entry")?;
//         tree_entries.push(tree_entry);
//     }

//     tree_entries.sort_unstable_by(|entry1, entry2| 
//         {compare_base_name(&entry1.name, &entry1.mode, &entry2.name, &entry2.mode)}
//     );

//     for tree_entry in tree_entries {
//         let hash = match tree_entry.mode {
//             EntryMode::Directory => {
//                 // If the entry is a directory, get the hash recursively.
//                 let Some(hash) = write_tree_for(&tree_entry.path.as_path())? else {
//                     // If the directory is empty, skip it.
//                     continue;
//                 };
//                 hash
//             }
//             _ => {
//                 // If the entry is not a directory, it will be a blob.
//                 let object = Object::blob_from_file(&tree_entry.path)
//                                                 .with_context(|| format!("Failed to create blob from file: {}", tree_entry.path.display()))?
//                                                 .hash()?;   

//             },
//         }
//     }
//     Ok([0;20])
// }


// pub(crate) fn invoke() -> anyhow::Result<()> {
//     let current_dir = std::env::current_dir()?;
//     write_tree_for(&current_dir)?;
//     Ok(())
// }