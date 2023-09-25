use crate::inputs::*;
use crate::utils::*;

use std::process::Command;

pub fn make_commit(commit_type: String, commit_message: String) {
    Command::new("git")
        .arg("commit")
        .arg("-S")
        .arg("-m")
        .arg(format!("{}: {}", commit_type, commit_message))
        .output()
        .expect("Failed to commit changes.");
}

pub fn git_add_all() {
    Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Failed to add files.");
}

pub fn make_initial_commit() {
    let commit_message_options: [&str; 2] = [
        "Use default initial commit message",
        "Create custom initial commit message",
    ];

    let commit_message_choice: Option<usize> = select_option(&commit_message_options);
    if commit_message_choice.is_none() {
        println!("Please choose a commit message option");
        std::process::exit(1);
    }

    let commit_message_choice_number: usize = commit_message_choice.unwrap();

    let commit_message_choice: String =
        commit_message_options[commit_message_choice_number].to_string();

    let commit_type: String;
    let commit_message: String;

    if commit_message_choice == "Create custom initial commit message" {
        commit_type = get_commit_type();
        commit_message = get_commit_message();
    } else {
        commit_type = String::from("chore");
        commit_message = String::from("initial commit");
    }

    git_add_all();
    make_commit(commit_type, commit_message);
}
