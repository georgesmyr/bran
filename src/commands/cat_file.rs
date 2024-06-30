use crate::objects;
use crate::objects::Object;
use anyhow::Context;

/// Invokes the `cat-file` subcommand.
///
/// # Arguments
///
/// * `pretty_print` - Pretty print flag.
/// * `object_hash` - Object hash to cat.
///
/// # Returns
///
/// * Contents of the object.
pub(crate) fn invoke(pretty_print: bool, object_hash: &str) -> anyhow::Result<()> {
    anyhow::ensure!(
        pretty_print,
        "Mode must be given without -p, and we don't support mode yet."
    );

    let (kind, size, reader) = objects::read_object(object_hash)?;

    match kind {
        objects::kind::ObjectKind::Blob => {
            let mut blob = objects::blob::Blob::new(size, reader);
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            let n = std::io::copy(&mut blob.content(), &mut stdout)
                .context("Failed to write to stdout.")?;
            anyhow::ensure!(
                n == blob.size(),
                "Object file did not have the expected size. Expected size: {}. Actual size: {}",
                blob.size(),
                n
            )
        }
        objects::kind::ObjectKind::Tree => {
            let tree_entries =
                objects::tree::Tree::from_hash(object_hash).context("Failed to read tree")?;
            for entry in tree_entries {
                println!("{}", entry);
            }
        }
        _ => anyhow::bail!("Object type not supported yet."),
    };

    Ok(())
}
