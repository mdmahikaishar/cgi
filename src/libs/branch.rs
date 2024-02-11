use std::process::{Command, ExitCode};

pub fn get_current_branch() -> ExitCode {
    let output = Command::new("git").arg("branch").arg("").output();

    if let Err(_) = output {
        return ExitCode::from(1);
    }

    println!("Branch: {:?}", output.unwrap().stdout);

    ExitCode::SUCCESS
}

pub fn create_branch(branch_name: String) -> ExitCode {
    let output = Command::new("git")
        .arg("branch")
        .arg("--create")
        .arg(branch_name.clone())
        .output();

    if let Err(_) = output {
        return ExitCode::from(1);
    }

    println!("Branch created \"{}\"", branch_name);

    ExitCode::SUCCESS
}

pub fn get_branch_list() -> ExitCode {
    let output = Command::new("git").arg("branch").arg("--list").output();

    if let Err(_) = output {
        return ExitCode::from(1);
    }

    println!("Branchs: {:?}", output.unwrap().stdout);

    ExitCode::SUCCESS
}

pub fn open_branch(branch_name: String) -> ExitCode {
    let output = Command::new("git")
        .arg("branch")
        .arg(branch_name.clone())
        .output();

    if let Err(_) = output {
        return ExitCode::from(1);
    }

    println!("Branch Opened \"{}\"", branch_name);

    ExitCode::SUCCESS
}
