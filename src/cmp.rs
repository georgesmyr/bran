use crate::objects::tree::mode::EntryMode;
use std::cmp::Ordering;
use std::ffi::OsStr;

/// Compares the base name of two entries.
///
/// This function compares the base name of two entries, taking into account their modes.
/// It returns an `Ordering` value that indicates the relative order of the entries.
///
/// # Arguments
///
/// * `name1` - The base name of the first entry.
/// * `mode1` - The mode of the first entry.
/// * `name2` - The base name of the second entry.
/// * `mode2` - The mode of the second entry.
///
/// # Returns
///
/// An `Ordering` value indicating the relative order of the entries.
pub(crate) fn compare_base_name(
    name1: &OsStr,
    mode1: &EntryMode,
    name2: &OsStr,
    mode2: &EntryMode,
) -> Ordering {
    let name1 = name1.as_encoded_bytes();
    let name2 = name2.as_encoded_bytes();
    let common_len = std::cmp::min(name1.len(), name2.len());

    match name1[..common_len].cmp(&name2[..common_len]) {
        Ordering::Equal => {}
        ord => return ord,
    }

    // If we are past the match expression, then the names are the same up to their common length.
    // If the lengths are the same, then the names are the same.
    if name1.len() == name2.len() {
        return Ordering::Equal;
    }

    // Check if we have reached the end of the name. If not, get the next character.
    // Otherwise, if the entry is a directory add the '/' character for the sake of comparing.
    // If the entry is not a directory, then we have reached the end of the name.
    let c1 = if let Some(c) = name1.get(common_len).copied() {
        Some(c)
    } else {
        match mode1 {
            EntryMode::Directory => Some(b'/'),
            _ => None,
        }
    };

    let c2 = if let Some(c) = name2.get(common_len).copied() {
        Some(c)
    } else {
        match mode2 {
            EntryMode::Directory => Some(b'/'),
            _ => None,
        }
    };

    // Compare the characters.
    c1.cmp(&c2)
}
