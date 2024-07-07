mod alias;
mod file_ops;

use alias::{add_alias, edit_alias, remove_alias, search_alias};
use dialoguer::{theme::ColorfulTheme, Select};
use dirs::home_dir;
use std::path::PathBuf;

fn main() {
    let alias_file = home_dir().map(|mut path: PathBuf| {
        path.push(".zsh_aliases");
        path
    });
    let alias_file = alias_file.as_deref().and_then(|path| path.to_str());
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
