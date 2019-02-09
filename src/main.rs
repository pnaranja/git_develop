//! Automate the command: git checkout develop; git pull; git branch -d <last branch>
//! This is used after the working branch has been merged

use std::process;
use std::process::Command;

fn main() {
    let get_cmd_result = |vec_output_result: Vec<u8>| {
        String::from_utf8(vec_output_result)
            .expect("error")
            .trim_end()
            .to_owned()
    };

    // Handle error if invalid command
    let handle_err = |msg| {
        eprintln!("ERROR running command: {:?}", msg);
        process::exit(1);
    };

    // Handle error when running valid command
    let handle_cmd_stderr = |output: std::process::Output| {
        if !output.status.success() {
            let err = get_cmd_result(output.stderr.to_owned());
            println!("Error: {}", err);
        }
    };

    let current_branch_pre = Command::new("git")
        .args(&["symbolic-ref", "--short", "HEAD"])
        .output()
        .unwrap_or_else(handle_err)
        .stdout
        .to_owned();

    // Get current branch
    // trim_end to remove the newline character
    let current_branch = get_cmd_result(current_branch_pre);

    handle_cmd_stderr(
        Command::new("git")
            .args(&["checkout", "develop"])
            .output()
            .unwrap_or_else(handle_err),
    );

    handle_cmd_stderr(
        Command::new("git")
            .args(&["pull"])
            .output()
            .unwrap_or_else(handle_err),
    );

    if current_branch == "develop" {
        eprintln!("Will not remove develop branch");
        process::exit(1);
    } else {
        println!("Removing branch: {}", current_branch);
        handle_cmd_stderr(
            Command::new("git")
                .args(&["push", "origin", "--delete", &current_branch])
                .output()
                .unwrap_or_else(handle_err),
        );
        handle_cmd_stderr(
            Command::new("git")
                .args(&["branch", "-d", &current_branch])
                .output()
                .unwrap_or_else(handle_err),
        );
    }
}
