mod objects;
mod commands;


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

    LsTree {
        /// List names only flag
        #[arg(short, long)]
        name_only: bool,

        /// Object hash to list
        object_hash: String,
    },

    // WriteTree 

}

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
        // Commands::WriteTree => commands::write_tree::invoke()?,
    }

    Ok(())
}
