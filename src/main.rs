mod alias;
mod file_ops;

use alias::{add_alias, edit_alias, remove_alias, search_alias};
use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    loop {
        let actions = vec!["Add", "Edit", "Remove", "Search", "Exit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select action")
            .default(0)
            .items(&actions)
            .interact()
            .unwrap();

        match actions[selection] {
            "Add" => add_alias(),
            "Edit" => edit_alias(),
            "Remove" => remove_alias(),
            "Search" => search_alias(),
            "Exit" => break,
            _ => unreachable!(),
        }
    }
}
