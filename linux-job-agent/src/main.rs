use std::env;
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let mut arguments = env::args().skip(1);
    let command = arguments.next();

    match command {
        Some(program) => {
            let result = Command::new(&program).args(arguments).status();
            match result {
                Ok(status) => match status.code() {
                    Some(code) => {
                        return ExitCode::from(code as u8);
                    }
                    None => {
                        return ExitCode::FAILURE;
                    }
                },
                Err(error) => {
                    println!("{}", error);
                }
            }
        }
        None => {
            println!("使い方: linux-job-agent <command> [args...]");
        }
    }
    ExitCode::FAILURE
}
