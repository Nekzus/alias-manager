use crate::file_ops::{self, Alias};
use console::Style;
use dialoguer::{theme::ColorfulTheme, Input, Select};

pub fn add_alias(file_path: Option<&str>) {
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
        name: alias_name.clone(),
        command: alias_command.clone(),
        description: alias_description.clone(),
    };

    if let Some(file_path_str) = file_path {
        if let Err(err) = file_ops::append_to_file(file_path_str, &alias) {
            eprintln!("Error adding alias: {}", err);
            return;
        }

        let style = Style::new().green();
        println!("{}", style.apply_to(format!("Alias added: {}", alias.name)));
        println!("Command: {}", alias.command);
        println!("Description: {}", alias.description);
        println!();
    } else {
        eprintln!("Error: file_path is None");
    }
}

pub fn edit_alias(file_path: Option<&str>) {
    if let Some(file_path_str) = file_path {
        let aliases = file_ops::load_aliases(file_path_str);
        let alias_names: Vec<_> = aliases.iter().map(|a| a.name.clone()).collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select alias to edit")
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
            command: new_command.clone(),
            description: new_description.clone(),
        };

        if let Err(err) = file_ops::update_alias(file_path_str, &new_alias) {
            eprintln!("Error updating alias: {}", err);
            return;
        }

        let style = Style::new().green();
        println!(
            "{}",
            style.apply_to(format!("Alias updated: {}", new_alias.name))
        );
        println!("New command: {}", new_alias.command);
        println!("New description: {}", new_alias.description);
        println!();
    } else {
        eprintln!("Error: file_path is None");
    }
}

pub fn remove_alias(file_path: Option<&str>) {
    if let Some(file_path_str) = file_path {
        let aliases = file_ops::load_aliases(file_path_str);
        let alias_names: Vec<_> = aliases.iter().map(|a| &a.name).collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select alias to remove")
            .items(&alias_names)
            .interact()
            .unwrap();

        let alias = &aliases[selection];

        if let Err(err) = file_ops::delete_alias(file_path_str, &alias.name) {
            eprintln!("Error removing alias: {}", err);
            return;
        }

        let style = Style::new().green();
        println!(
            "{}",
            style.apply_to(format!("Alias removed: {}", alias.name))
        );
        println!();
    } else {
        eprintln!("Error: file_path is None");
    }
}

pub fn search_alias(file_path: Option<&str>) {
    if let Some(file_path_str) = file_path {
        let aliases = file_ops::load_aliases(file_path_str);
        let alias_names: Vec<_> = aliases.iter().map(|a| &a.name).collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select alias to search")
            .items(&alias_names)
            .interact()
            .unwrap();

        let alias = &aliases[selection];

        let style = Style::new().cyan();
        println!("{}", style.apply_to(format!("Alias found: {}", alias.name)));
        println!("Command: {}", alias.command);
        println!("Description: {}", alias.description);
        println!();
    } else {
        eprintln!("Error: file_path is None");
    }
}
