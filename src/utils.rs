use std::fs::File;
use std::io::BufRead;
use crate::pentry::Vault;

pub fn println(token: &str, prefix: Option<&str>) {
    if let Some(prefix) = prefix {
        println!("{prefix}{token}");
    } else {
        println!("{token}");
    }
}


pub fn read_passwords_file() -> Result<Vec<Vault>, std::io::Error> {
    let file = File::open("credentials.json")?;
    let reader = std::io::BufReader::new(file);

    let mut stored = Vec::new();

    for line in reader.lines() {
        if let Ok(json_str) = line {
            if let Ok(vault) = Vault::from_json(&json_str) {
                stored.push(vault);
            }
        }
    }

    Ok(stored)
}