use std::process::{Command, ExitCode};
use crate::utils::get_random_message;

pub fn commit_with_message(message: String) -> ExitCode {
    let output = Command::new("git").arg("-m").arg(message.clone()).output();

    if let Err(_) = output {
        return ExitCode::from(1);
    }

    println!("Commited \"{}\"", message);

    ExitCode::SUCCESS
}

pub fn commit_random_message() -> ExitCode {
    let random_message = get_random_message();

    commit_with_message(random_message)
}
