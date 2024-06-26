use clap;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct GitCLI {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(clap::Subcommand)]
pub(crate) enum Commands {
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

    WriteTree {
        tree_path: Option<String>,
    },

    CommitTree {
        tree_hash: String,

        #[arg(short = 'p', long)]
        parent_hash: Option<String>,

        #[arg(short = 'm', long)]
        message: String,
    },

    LsFiles,

    Add {
        path: String,
    },
}
