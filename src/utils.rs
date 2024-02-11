use std::path::PathBuf;

pub fn get_git_dir() -> PathBuf {
    let mut current_dir = std::env::current_dir().unwrap();
    current_dir.push(".git");

    current_dir
}

pub fn get_random_message() -> String {
    "Hello".to_string()
}
