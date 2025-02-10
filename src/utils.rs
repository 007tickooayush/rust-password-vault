use std::fs::File;
use std::io;
use std::io::{BufRead, Read};
use crate::pentry::Vault;

pub fn println(token: &str, prefix: Option<&str>) {
    if let Some(prefix) = prefix {
        println!("{prefix}{token}");
    } else {
        println!("{token}");
    }
}

pub fn read_passwords_file<'a>() -> Result<Vec<Vault>, std::io::Error> {
    let file = File::open("credentials.json")?;
    let reader = std::io::BufReader::new(file);

    let mut credentials = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let vault = Vault::from_json(line)?;
            credentials.push(vault);
        }
    }

    Ok(credentials)
}

pub fn search_passwords_file(search_term: &str) -> Result<Vec<Vault>, std::io::Error> {
    let file = File::open("credentials.json")?;
    let reader = std::io::BufReader::new(file);

    let mut results = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let matches: Vec<&str> = line.matches(format!(r#".+:"{}""#, search_term).as_str()).collect();

            if !matches.is_empty() {
                let vault = Vault::from_json(line)?;
                results.push(vault);
            }
        }
    }

    println!("\n\n");
    if results.is_empty() {
        println(format!("NO MATCHES FOUND for {search_term}").as_str(), None);
    }

    Ok(results)
}

#[test]
fn test_read_passwords_file() {
    let result = read_passwords_file().unwrap();
    assert!(!result.is_empty());

    println!("{:?}", result);

    let file = File::open("credentials.json").unwrap();
    let mut reader = std::io::BufReader::new(file);

    let mut result = String::new();
    reader.read_to_string(&mut result).unwrap();

    println!("{:?}", result);
}