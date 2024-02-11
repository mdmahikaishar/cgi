use crate::utils::get_git_dir;
use std::fs;
use std::process::{Command, ExitCode};

pub fn init() -> ExitCode {
    let output = Command::new("git").arg("init").output();

    if let Err(_) = output {
        return ExitCode::from(1);
    }

    println!("Initilized Git Rep.");

    ExitCode::SUCCESS
}

pub fn uninit() -> ExitCode {
    let location = get_git_dir();

    if !location.exists() {
        return ExitCode::from(1);
    }

    if let Err(_) = fs::remove_dir_all(location) {
        return ExitCode::from(1);
    }

    println!("UnInitilized Git Rep.");

    ExitCode::SUCCESS
}
