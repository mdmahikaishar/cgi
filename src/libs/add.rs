use std::process::{Command, ExitCode};

pub fn add(props: &[String]) -> ExitCode {
    let output = Command::new("git").arg("add").arg(props.join(" ")).output();

    if let Err(_) = output {
        return ExitCode::from(1);
    }

    println!("Added \"{}\"", props.join(" "));

    ExitCode::SUCCESS
}
