mod cmp;
mod commands;
mod config;
mod index;
mod objects;

use crate::commands::cli::{Commands, GitCLI};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = GitCLI::parse();

    match args.command {
        // Initialize repository
        Commands::Init { path } => {
            if let Some(path) = path {
                commands::init::invoke(&path);
            } else {
                commands::init::invoke(".")
            }
        }

        // Cat file by object ID
        Commands::CatFile {
            pretty_print,
            object_hash,
        } => {
            commands::cat_file::invoke(pretty_print, &object_hash)?;
        }

        // Hash object, optionally write to file
        Commands::HashObject { write, file } => commands::hash_object::invoke(&file, write)?,

        // List tree by object ID
        Commands::LsTree {
            name_only,
            object_hash,
        } => commands::ls_tree::invoke(&object_hash, name_only)?,
        // Write tree
        Commands::WriteTree { tree_path } => match tree_path {
            Some(path) => commands::write_tree::invoke(std::path::Path::new(&path))?,
            None => commands::write_tree::invoke(std::path::Path::new("."))?,
        },

        // Commit tree
        Commands::CommitTree {
            tree_hash,
            parent_hash,
            message,
        } => {
            commands::commit_tree::invoke(tree_hash, parent_hash, message)?;
        }

        // List files in index
        Commands::LsFiles => commands::ls_files::invoke(".git/index")?,

        // Add file to index
        Commands::Add { path } => commands::add::invoke(&path)?,
    }

    Ok(())
}
