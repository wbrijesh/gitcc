use dialoguer::{theme::ColorfulTheme, Select};

pub fn select_option(prompt: &str, options: &[&str]) -> Option<usize> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(options)
        .interact_opt()
        .unwrap();

    if selection.is_none() {
        println!("Please choose an option");
        std::process::exit(1);
    }

    return selection;
}

pub fn select_option_string_vec(prompt: &str, options: &Vec<String>) -> Option<usize> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(&options)
        .interact_opt()
        .unwrap();

    if selection.is_none() {
        println!("Please choose an option");
        std::process::exit(1);
    }

    return selection;
}
