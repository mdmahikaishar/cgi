use std::process::{Command, ExitCode};

pub fn merge() -> ExitCode {
    let output = Command::new("git").arg("merge").output();

    if let Err(_) = output {
        return ExitCode::from(1);
    }

    println!("Merge {:?}", output.unwrap().stdout);

    ExitCode::SUCCESS
}
