use libs::{add, branch, commit, init, log, merge,help, origin, push, status};
use std::env;
use std::process::ExitCode;

mod libs;
mod utils;

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        eprintln!("Need more args.");
        return ExitCode::from(1);
    }

    match &args[1][..] {
        "help" | "-h" => help::help(),
        "init" | "-i" => init::init(),
        "uninit" | "-u" => init::uninit(),
        "commit" | "-c" => {
            if args.len() < 3 {
                eprintln!("Commit need more args.");
                return ExitCode::from(1);
            }

            match &args[2][..] {
                // Commit with message
                "--message" | "-m" => {
                    if args.len() < 4 {
                        eprintln!("Commit -m need one more arg.");
                        return ExitCode::from(1);
                    }

                    commit::commit_with_message(args[3].clone())
                }
                // Commit random message
                "--random" | "-r" => commit::commit_random_message(),
                // ERROR
                _ => {
                    eprintln!("Invalid commit args.");
                    return ExitCode::from(1);
                }
            }
        }
        "add" | "-a" => {
            if args.len() < 3 {
                eprintln!("Add needs more args.");
                return ExitCode::from(1);
            }

            add::add(&args[2..])
        }
        "origin" | "-o" => {
            if args.len() < 3 {
                return origin::get_origin();
            }

            return origin::add_origin(args[2].clone());
        }
        "branch" | "-b" => {
            if args.len() < 3 {
                return branch::get_current_branch();
            }

            match &args[2][..] {
                // CREATE BRANCH
                "--create" | "-c" => {
                    if args.len() < 4 {
                        println!("ERROR: failed to create new branch.");
                        return ExitCode::from(1);
                    }

                    branch::create_branch(args[3].clone())
                }

                // BRANCH LIST
                "--list" | "-l" => branch::get_branch_list(),

                // GO TO BRANCH
                _ => branch::open_branch(args[2].clone()),
            }
        }
        "push" | "-p" => {
            if args.len() < 4 {
                println!("ERROR: `push` needs more args.");
                return ExitCode::from(1);
            }

            return push::push_upstream(args[2].clone(), args[3].clone());
        }
        "status" | "-s" => status::status(),
        "log" | "-l" => log::log(),
        "merge" | "-m" => merge::merge(),
        _ => {
            help::help();
            return ExitCode::from(1);
        }
    }
}
