use std::fs;
use std::path::Path;

/// Invokes the `init` subcommand.
///
/// # Arguments
///
/// * `path` - Path of the repository to initialize.
///
pub fn invoke(path: &str) {
    let mut repo_dir = Path::new(path).join(".git");

    if repo_dir.exists() {
        repo_dir = fs::canonicalize(repo_dir).unwrap();
        println!(
            "Reinitialized existing Git repository in {}",
            repo_dir.display()
        );
    } else {
        fs::create_dir_all(&repo_dir).unwrap();
        repo_dir = fs::canonicalize(repo_dir).unwrap();

        let objects_dir = repo_dir.join("objects");
        let refs_dir = repo_dir.join("refs");
        let head_path = repo_dir.join("HEAD");

        fs::create_dir(&objects_dir).unwrap();
        fs::create_dir(&refs_dir).unwrap();
        fs::write(&head_path, "ref: refs/heads/main\n").unwrap();

        println!("Initialized empty Git repository in {}", repo_dir.display());
    }
}
