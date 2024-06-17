use crate::kind::ObjectKind;
use crate::object::Object;
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

    let mut object = Object::read(object_hash)?;

    match object.kind {
        ObjectKind::Blob => {
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            let n = std::io::copy(&mut object.reader, &mut stdout)
                .context("write .git/objects file to stdout")?;

            anyhow::ensure!(
                n == object.size,
                ".git/object file was not the expected size (expected: {}, actual: {n})",
                object.size
            );
        }
        _ => anyhow::bail!("Object type not supported yet."),
    };

    Ok(())
}
