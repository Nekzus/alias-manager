use crate::file_ops::{append_to_file, delete_alias, load_aliases, update_alias};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use serde::{Deserialize, Serialize};

const ALIAS_FILE: &str = "/home/nekzus-dev/.zsh_aliases";

#[derive(Serialize, Deserialize, Debug)]
pub struct Alias {
    pub name: String,
    pub command: String,
    pub description: String,
}

pub fn add_alias() {
    let alias_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter alias name")
        .interact_text()
        .unwrap();

    let alias_command: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter command for alias")
        .interact_text()
        .unwrap();

    let alias_description: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter description for alias")
        .interact_text()
        .unwrap();

    let alias = Alias {
        name: alias_name,
        command: alias_command,
        description: alias_description,
    };

    let alias_line = format!(
        "alias {}='{}' # {}\n",
        alias.name, alias.command, alias.description
    );
    append_to_file(ALIAS_FILE, &alias_line).unwrap();
    println!("Alias added: {}", alias_line);
}

pub fn edit_alias() {
    let aliases = load_aliases(ALIAS_FILE);
    let alias_names: Vec<_> = aliases.iter().map(|a| a.name.clone()).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select alias to edit")
        .default(0)
        .items(&alias_names)
        .interact()
        .unwrap();

    let alias = &aliases[selection];

    let new_command: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter new command")
        .default(alias.command.clone())
        .interact_text()
        .unwrap();

    let new_description: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter new description")
        .default(alias.description.clone())
        .interact_text()
        .unwrap();

    let new_alias = Alias {
        name: alias.name.clone(),
        command: new_command,
        description: new_description,
    };

    update_alias(ALIAS_FILE, &new_alias).unwrap();
    println!("Alias updated: {:?}", new_alias);
}

pub fn remove_alias() {
    let aliases = load_aliases(ALIAS_FILE);
    let alias_names: Vec<_> = aliases.iter().map(|a| &a.name).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select alias to remove")
        .default(0)
        .items(&alias_names)
        .interact()
        .unwrap();

    let alias = &aliases[selection];
    delete_alias(ALIAS_FILE, &alias.name).unwrap();
    println!("Alias removed: {}", alias.name);
}

pub fn search_alias() {
    let aliases = load_aliases(ALIAS_FILE);
    let alias_names: Vec<_> = aliases.iter().map(|a| &a.name).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select alias to search")
        .default(0)
        .items(&alias_names)
        .interact()
        .unwrap();

    let alias = &aliases[selection];
    println!("Alias found: {:?}", alias);
}
