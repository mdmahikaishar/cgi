use std::process::{Command, ExitCode};

pub fn log() -> ExitCode {
    let output = Command::new("git").arg("log").output();

    if let Err(_) = output {
        return ExitCode::from(1);
    }

    println!("Log: \n{:?}", output.unwrap().stdout);

    ExitCode::SUCCESS
}
