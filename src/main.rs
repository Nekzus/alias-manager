mod alias;
mod file_ops;

use alias::{add_alias, edit_alias, remove_alias, search_alias};
use dialoguer::{theme::ColorfulTheme, Select};

const ALIAS_FILE: &str = "/home/nekzus-dev/.zsh_aliases";

fn main() {
    loop {
        let actions = vec!["Add", "Edit", "Remove", "Search", "Exit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select action")
            .items(&actions)
            .interact()
            .unwrap();

        match actions[selection] {
            "Add" => add_alias(ALIAS_FILE),
            "Edit" => edit_alias(ALIAS_FILE),
            "Remove" => remove_alias(ALIAS_FILE),
            "Search" => search_alias(ALIAS_FILE),
            "Exit" => break,
            _ => println!("Invalid option"),
        }
    }
}
