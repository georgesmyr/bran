use std::env;
use std::path::PathBuf;

/// A helper struct to reset the working directory after a test.
pub struct ResetWorkingDir(PathBuf);

impl ResetWorkingDir {
    /// Create a new instance of ResetWorkingDir.
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }
}

/// When the test ends and the ResetWorkingDir instance goes out of scope, the Drop trait implementation will be called,
/// resetting the working directory to its original state.
impl Drop for ResetWorkingDir {
    fn drop(&mut self) {
        env::set_current_dir(&self.0).expect("Could not reset working directory");
        println!("Reset working directory to: {:?}", self.0);
    }
}
