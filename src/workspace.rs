use std::path::PathBuf;

const REPO_DIRNAME: &str = ".git";

pub struct Workspace {
    pub dir: PathBuf,
    pub ignore_list: Vec<PathBuf>,
}

#[allow(dead_code)]
impl Workspace {
    pub fn new(path: &PathBuf) -> Self {
        if let Some(git_dir) = find_git_dir(path.clone()) {
            let dir = git_dir.parent().unwrap().to_path_buf();
            let ignore_list = vec![dir.join(REPO_DIRNAME)];
            Workspace { dir, ignore_list }
        } else {
            panic!("fatal: not a git repository (or any of the parent directories): .git");
        }
    }

    pub fn list_repo(&self) -> Vec<PathBuf> {
        let mut list = vec![];

        let entries = self.dir.read_dir().unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if self.ignore_list.contains(&path) {
                continue;
            }
            println!("{:?}", path);
            list.push(path);
        }
        list
    }
}

/// Finds if .git directory exists in the specified path.
pub fn find_git_dir(mut path: PathBuf) -> Option<PathBuf> {
    loop {
        // Check if the .git directory exists in the current path
        let git_path = path.join(REPO_DIRNAME);
        if git_path.exists() && git_path.is_dir() {
            return Some(git_path);
        }

        // Move up to the parent directory, if possible
        if !path.pop() {
            // If no parent is found, we're at the root, and no .git directory was found
            return None;
        }
    }
}
