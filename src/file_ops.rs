use std::fs::{self, OpenOptions};
use std::io::{self, Write};

pub struct Alias {
    pub name: String,
    pub command: String,
    pub description: String,
}

pub fn append_to_file(file_path: &str, alias: &Alias) -> io::Result<()> {
    let alias_line = format!(
        "alias {}='{}' # {}\n",
        alias.name, alias.command, alias.description
    );

    let mut file = OpenOptions::new().append(true).open(file_path)?;
    file.write_all(alias_line.as_bytes())?;
    Ok(())
}

pub fn load_aliases(file_path: &str) -> Vec<Alias> {
    match fs::read_to_string(file_path) {
        Ok(content) => content
            .lines()
            .filter_map(|line| {
                if line.starts_with("alias ") {
                    let parts: Vec<&str> = line.split('=').collect();
                    if parts.len() != 2 {
                        return None;
                    }
                    let name = parts[0][6..].trim().to_string();
                    let command_description: Vec<&str> = parts[1].split('#').collect();
                    if command_description.len() != 2 {
                        return None;
                    }
                    let command = command_description[0].trim_matches('\'').trim().to_string();
                    let description = command_description[1].trim().to_string();
                    Some(Alias {
                        name,
                        command,
                        description,
                    })
                } else {
                    None
                }
            })
            .collect(),
        Err(_) => Vec::new(),
    }
}

pub fn update_alias(file_path: &str, new_alias: &Alias) -> io::Result<()> {
    let mut aliases = load_aliases(file_path);

    if let Some(alias) = aliases.iter_mut().find(|a| a.name == new_alias.name) {
        alias.command = new_alias.command.clone();
        alias.description = new_alias.description.clone();
    }

    save_aliases(file_path, aliases.as_slice())
}

pub fn delete_alias(file_path: &str, alias_name: &str) -> io::Result<()> {
    let mut aliases = load_aliases(file_path);
    aliases.retain(|a| a.name != alias_name);
    save_aliases(file_path, &aliases[..])
}

fn save_aliases(file_path: &str, aliases: &[Alias]) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;

    for alias in aliases {
        let alias_line = format!(
            "alias {}='{}' # {}\n",
            alias.name, alias.command, alias.description
        );
        file.write_all(alias_line.as_bytes())?;
    }

    Ok(())
}
