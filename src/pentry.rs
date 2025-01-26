use std::io;
use std::io::Write;
use serde::{Deserialize, Serialize};
use crate::utils::println;

#[derive(Debug, Serialize, Deserialize)]
pub struct Vault<'pass> {
    pub service: &'pass str,
    pub username: &'pass str,
    pub password: &'pass str,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Vault {
//     pub service: String,
//     pub username: String,
//     pub password: String
// }

impl<'pass> Vault<'pass> {
    pub fn new(service: &'pass str, username: &'pass str, password: &'pass str) -> Self {
        Vault {
            service,
            username,
            password
        }
    }


}

/// Function to create prompt using input string
/// NOTE: can not be tested as it requires user input from stdin
///
pub fn prompt<'a>(prompt: &'a mut String, prefix: Option<&'a str>) -> &'a str {
    println(prompt, prefix);
    io::stdout().flush().unwrap();

    // let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();
    // input.trim()

    io::stdin().read_line(prompt).unwrap();
    prompt.trim()
}

pub fn read_passwords_file() {}


#[test]
fn test_new_vault() {
    let vault = Vault::new("aws-s3", "ceo", "ceo");
    println!("{:?}", vault);
}