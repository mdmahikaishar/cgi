use std::process::{Command, ExitCode};

pub fn status() -> ExitCode {
    let output = Command::new("git").arg("status").output();

    if let Err(_) = output {
        return ExitCode::from(1);
    }

    println!("Status: \n{:?}", output.unwrap().stdout);

    ExitCode::SUCCESS
}
