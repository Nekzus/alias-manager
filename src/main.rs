mod alias;
mod file_ops;

use alias::{add_alias, edit_alias, remove_alias, search_alias};
use dialoguer::{theme::ColorfulTheme, Select};
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let alias_file = env::var("ALIAS_FILE").expect("ALIAS_FILE not set");
    let alias_file = alias_file.as_str();
    loop {
        let actions = vec!["Add", "Edit", "Remove", "Search", "Exit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select action")
            .items(&actions)
            .interact()
            .unwrap();

        match actions[selection] {
            "Add" => add_alias(alias_file),
            "Edit" => edit_alias(alias_file),
            "Remove" => remove_alias(alias_file),
            "Search" => search_alias(alias_file),
            "Exit" => break,
            _ => println!("Invalid option"),
        }
    }
}
