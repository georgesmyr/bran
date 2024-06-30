use anyhow::Context;

use crate::objects::kind::ObjectKind;
use crate::objects::read_object;
use crate::objects::tree::Tree;

pub(crate) fn invoke(hash: &str, name_only: bool) -> anyhow::Result<()> {
    let (kind, _, _) = read_object(hash).context("Failed to read object")?;

    let tree_entries = match kind {
        ObjectKind::Tree => Tree::from_hash(hash).context("Failed to read tree")?,
        _ => anyhow::bail!("Object is not a tree."),
    };

    for entry in tree_entries {
        if name_only {
            println!("{}", entry.name().to_string_lossy());
        } else {
            println!("{}", entry);
        }
    }

    Ok(())
}
