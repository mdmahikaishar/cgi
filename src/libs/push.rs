use std::process::{Command, ExitCode};

pub fn push_upstream(src: String, dest: String) -> ExitCode {
    let output = Command::new("git")
        .arg("push")
        .arg("-u")
        .arg(src.clone())
        .arg(dest.clone())
        .output();

    if let Err(_) = output {
        return ExitCode::from(1);
    }

    println!("Pushed \"{}\" to \"{}\"", src, dest);

    ExitCode::SUCCESS
}
