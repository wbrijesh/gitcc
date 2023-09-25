use git2::Repository;
use std::env::current_dir;
use std::process::Command;

use crate::inputs::*;
use crate::utils::*;

pub fn make_commit(commit_type: String, commit_message: String) {
    Command::new("git")
        .arg("commit")
        .arg("-S")
        .arg("-m")
        .arg(format!("{}: {}", commit_type, commit_message))
        .output()
        .expect("Failed to commit changes.");
}

pub fn confirm_and_stage_all() {
    let staging_options: [&str; 2] = ["Yes", "No"];
    let staging_choice_number: Option<usize> =
        select_option("Would you like to stage all files? ", &staging_options);
    let staging_choice: String = staging_options[staging_choice_number.unwrap()].to_string();

    if staging_choice == "Yes" {
        Command::new("git")
            .arg("add")
            .arg("-A")
            .output()
            .expect("Failed to add files.");
    }
}

// function to push to remote
pub fn confirm_and_push_to_remote() {
    let push_options: [&str; 2] = ["Yes", "No"];
    let push_choice_number: Option<usize> =
        select_option("Would you like to push to remote? ", &push_options);
    let push_choice: String = push_options[push_choice_number.unwrap()].to_string();

    if push_choice == "Yes" {
        let repo: Repository = Repository::open(current_dir().unwrap()).unwrap();
        let remotes_string_array = repo.remotes().unwrap();

        let mut remotes: Vec<String> = Vec::new();
        for remote in &remotes_string_array {
            remotes.push(remote.unwrap().to_string());
        }

        let remote_choice_number: Option<usize> =
            select_option_string_vec("Select a remote to push to ", &remotes);
        let remote_choice: String = remotes[remote_choice_number.unwrap()].to_string();

        // TODO: ask user to choose branch

        println!("Pushing to {}...", remote_choice);

        Command::new("git")
            .arg("push")
            .arg(remote_choice)
            .output()
            .expect("Failed to push to remote.");
    }
}

pub fn make_initial_commit() {
    let commit_message_options: [&str; 2] = ["Use default", "Create custom"];

    let commit_message_choice_number: Option<usize> =
        select_option("Choose initial commit message ", &commit_message_options);

    let commit_message_choice: String =
        commit_message_options[commit_message_choice_number.unwrap()].to_string();

    let commit_type: String;
    let commit_message: String;

    if commit_message_choice == "Create custom" {
        commit_type = get_commit_type();
        commit_message = get_commit_message();
    } else {
        commit_type = String::from("chore");
        commit_message = String::from("initial commit");
    }

    confirm_and_stage_all();
    make_commit(commit_type, commit_message);
}
