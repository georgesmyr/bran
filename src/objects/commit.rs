use crate::objects;
use crate::objects::Object;
use anyhow::Context;
use std::fmt::Write;

pub(crate) struct Commit<R> {
    kind: objects::kind::ObjectKind,
    size: u64,
    reader: R,
}

impl Commit<()> {
    /// Creates new `Commit` object.
    ///
    /// # Arguments
    ///
    /// * `tree` - The tree hash.
    /// * `parent` - The parent hash.
    /// * `author_name` - The author name.
    /// * `author_email` - The author email.
    /// * `committer_name` - The committer name.
    /// * `committer_email` - The committer email.
    /// * `message` - The commit message.
    ///
    /// # Returns
    ///
    /// Returns a `Commit` object.
    pub(crate) fn new(
        tree: objects::id::ObjectID,
        parent: Option<objects::id::ObjectID>,
        author_name: String,
        author_email: String,
        committer_name: String,
        committer_email: String,
        message: String,
    ) -> anyhow::Result<Commit<impl std::io::Read>> {
        let mut commit = String::new();
        // Write the tree and parent hashes (if any)
        writeln!(commit, "tree {}", tree.hash()).context("Failed to write tree hash")?;
        if let Some(parent) = parent {
            writeln!(commit, "parent {}\n", parent.hash())
                .context("Failed to write parent hash")?;
        };

        // Write the author and committer information with the current timestamp
        let local = chrono::Local::now();
        let timestamp = local.timestamp();
        let timezone = format!("{}", local.offset()).replace(":", "");
        writeln!(
            commit,
            "author {} <{}> {} {}",
            author_name, author_email, timestamp, timezone
        )
        .context("Failed to write author information")?;
        writeln!(
            commit,
            "committer {} <{}> {} {}",
            committer_name, committer_email, timestamp, timezone
        )
        .context("Failed to write committer information")?;

        // Write the commit message
        writeln!(commit, "\n{}", message).context("Failed to write commit message")?;

        Ok(Commit {
            kind: objects::kind::ObjectKind::Commit,
            size: commit.len() as u64,
            reader: std::io::Cursor::new(commit),
        })
    }
}

impl<R: std::io::Read> Object for Commit<R> {
    fn kind(&self) -> &objects::kind::ObjectKind {
        &self.kind
    }

    fn size(&self) -> u64 {
        self.size
    }

    fn content(&mut self) -> &mut dyn std::io::Read {
        &mut self.reader
    }
}
