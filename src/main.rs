mod commands;
mod hashwriter;
mod kind;
mod object;
mod oid;
mod workspace;
// use workspace::Workspace;

// use std::env;
// use object::Object;

// use std::path::{Path, PathBuf};
use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct GitCLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        /// Optional path to initialize
        #[arg(short, long)]
        path: Option<String>,
    },

    CatFile {
        /// Pretty print the object
        #[arg(short, long)]
        pretty_print: bool,

        /// Object hash to cat
        object_hash: String,
    },

    HashObject {
        /// Write the object to the object database
        #[arg(short, long)]
        write: bool,

        /// File to hash
        file: String,
    },
}

fn main() -> anyhow::Result<()> {
    // let cwd = env::current_dir().unwrap();
    // let path = cwd.join("xxx");

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
    }
    // let ws = Workspace::new(&path);
    // let repo_list = ws.list_repo();

    // for p in &repo_list {
    //     let mut blob = Object::blob_from_file(p).unwrap();
    //     let hash = blob.hash()?.to_string();
    //     let mut blob2 = Object::read(&hash)?;

    //     let string = blob2.contents().unwrap();
    //     println!("{:?}", string);

    // }

    Ok(())
}
