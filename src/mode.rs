use std::fs::Metadata;
use std::os::unix::fs::PermissionsExt;

/// Determines the mode of the file based on its metadata.
///
/// The mode is represented as a string value. Possible values are:
/// - "040000" for directories
/// - "120000" for symbolic links
/// - "100755" for executable files
/// - "100644" for non-executable files
///
/// # Arguments
///
/// * `metadata` - The metadata of the file.
///
/// # Returns
///
/// The mode of the file as a string.
pub(crate) fn get_mode(metadata: Metadata) -> &'static str {
    let mode = if metadata.is_dir() {
        "040000"
    } else if metadata.is_symlink() {
        "120000"
    } else if metadata.permissions().mode() & 0o111 != 0 {
        "100755" 
    } else {
        "100644"
    };
    mode
}