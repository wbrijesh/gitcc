use dialoguer::{theme::ColorfulTheme, Select};

pub fn select_option(options: &[&str]) -> Option<usize> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option:")
        .default(0)
        .items(options)
        .interact_opt()
        .unwrap();

    return selection;
}
