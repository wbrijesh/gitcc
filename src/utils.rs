use dialoguer::{theme::ColorfulTheme, Select};

pub fn select_option(options: &[&str]) -> Option<usize> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option:")
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
