mod cli;
mod cmp;
mod commands;
mod index;
mod objects;

use crate::objects::Object;

use crate::cli::{Commands, GitCLI};
use clap::Parser;

use std::path::{Path, PathBuf};

fn main() -> anyhow::Result<()> {
    // let index_path = Path::new(".git/index");
    // let file_path = Path::new("world.txt");
    // let mut index = index::Index::init(index_path)?;
    // let mut blob = objects::blob::Blob::from_file(file_path)?;
    // let file = std::fs::File::open(file_path)?;
    // let meta = file.metadata()?;
    // let oid = blob.hash()?;
    // let entry = index::IndexEntry::new(file_path.to_path_buf(), oid, &meta);
    // index.add(entry);

    // for entry in index.entries() {
    //     println!("{:?}", entry);
    // }

    // index.write("new_index")?;
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
            Some(path) => commands::write_tree::invoke(Path::new(&path))?,
            None => commands::write_tree::invoke(Path::new("."))?,
        },
        _ => unimplemented!(),
    }

    Ok(())
}
