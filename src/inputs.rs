use crate::utils::select_option;

pub fn get_commit_type() -> String {
    let commit_type_options: [&str; 8] = [
        "feat", "fix", "docs", "style", "refactor", "test", "chore", "revert",
    ];

    let commit_type_choice: Option<usize> = select_option(&commit_type_options);

    if commit_type_choice.is_none() {
        println!("Please choose a commit type");
        std::process::exit(1);
    }

    let commit_type_choice: String = commit_type_options[commit_type_choice.unwrap()].to_string();

    return commit_type_choice;
}

pub fn get_commit_message() -> String {
    println!("Please enter a commit message: ");

    let mut commit_message: String = String::new();
    std::io::stdin()
        .read_line(&mut commit_message)
        .expect("Failed to read line");

    return commit_message;
}
