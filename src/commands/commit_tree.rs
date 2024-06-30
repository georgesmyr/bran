use crate::config;
use crate::objects;
use crate::objects::Object;
use anyhow::Context;

pub(crate) fn invoke(
    tree_hash: String,
    parent_hash: Option<String>,
    message: String,
) -> anyhow::Result<()> {
    let config = config::Config::load();

    // Create a new commit object
    let parent_hash = match parent_hash {
        Some(hash) => Some(objects::id::ObjectID::from_hash(&hash)),
        None => None,
    };
    let mut commit = objects::commit::Commit::new(
        objects::id::ObjectID::from_hash(&tree_hash),
        parent_hash,
        config.author_name.clone(),
        config.author_email.clone(),
        config.author_name,
        config.author_email,
        message,
    )
    .context("Failed to create commit")?;

    // Write the commit object to the database
    let oid = commit.write()?;
    println!("{}", oid.hash());

    Ok(())
}
