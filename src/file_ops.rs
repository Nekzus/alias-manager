use crate::alias::Alias;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};

pub fn append_to_file(path: &str, text: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

pub fn load_aliases(path: &str) -> Vec<Alias> {
    let file = OpenOptions::new().read(true).open(path).unwrap();
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .filter_map(|line| {
            if let Ok(line) = line {
                if line.starts_with("alias ") {
                    let parts: Vec<&str> = line.split("=").collect();
                    let name = parts[0][6..].to_string();
                    let command_description: Vec<&str> = parts[1].split("#").collect();
                    let command = command_description[0].trim_matches('\'').to_string();
                    let description = command_description[1].trim().to_string();
                    return Some(Alias {
                        name,
                        command,
                        description,
                    });
                }
            }
            None
        })
        .collect()
}

pub fn update_alias(path: &str, new_alias: &Alias) -> io::Result<()> {
    let aliases = load_aliases(path);
    let updated_aliases: Vec<Alias> = aliases
        .into_iter()
        .map(|alias| {
            if alias.name == new_alias.name {
                Alias {
                    name: new_alias.name.clone(),
                    command: new_alias.command.clone(),
                    description: new_alias.description.clone(),
                }
            } else {
                alias
            }
        })
        .collect();

    save_aliases(path, &updated_aliases)
}

pub fn delete_alias(path: &str, alias_name: &str) -> io::Result<()> {
    let aliases = load_aliases(path);
    let filtered_aliases: Vec<Alias> = aliases
        .into_iter()
        .filter(|alias| alias.name != alias_name)
        .collect();

    save_aliases(path, &filtered_aliases)
}

pub fn save_aliases(path: &str, aliases: &[Alias]) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;
    for alias in aliases {
        let alias_line = format!(
            "alias {}='{}' # {}\n",
            alias.name, alias.command, alias.description
        );
        file.write_all(alias_line.as_bytes())?;
    }
    Ok(())
}
