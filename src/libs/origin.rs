use std::process::{Command, ExitCode};

pub fn get_origin() -> ExitCode {
    let output = Command::new("git")
        .arg("remote")
        .arg("origin")
        .arg("--GET")
        .output();

    if let Err(_) = output {
        return ExitCode::from(1);
    }

    println!("Origin: {:?}", output.unwrap().stdout);

    ExitCode::SUCCESS
}

pub fn add_origin(uri: String) -> ExitCode {
    let output = Command::new("git")
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg(uri.clone())
        .output();

    if let Err(_) = output {
        return ExitCode::from(1);
    }

    println!("Origin added \"{}\"", uri);

    ExitCode::SUCCESS
}
