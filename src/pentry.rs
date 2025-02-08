use std::fs::OpenOptions;
use std::io;
use std::io::{Error, ErrorKind, Write};
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

    fn from_json(json_string: &'pass str) -> Result<Self, serde_json::Error> {
        // todo!("Implement this function");
        // unimplemented!("Implement this function")
        serde_json::from_str(json_string)
    }

    fn to_json(&self) -> String {
        // todo!("Implement this function");
        // unimplemented!("Implement this function")
        serde_json::to_string(&self).expect("Error converting to JSON")
    }

    pub fn write_to_file(&self) -> Result<(), io::Error> {
        let json_formatted = format!("{}\n",self.to_json());

        match OpenOptions::new()
            .create(true)
            .append(true)
            .open("credentials.json") {
            Ok(mut file) => {
                if let Err(error) = file.write_all(json_formatted.as_bytes()) {
                    println(&error.to_string(), None);
                    Err(error)
                }  else {
                    Ok(())
                }

            },
            Err(e) => {
                println(&e.to_string(),None);
                Err(e)
            }
        }
    }

}


pub fn read_passwords_file() {
    todo!("define the function properly")
}


#[test]
fn test_new_vault() {
    let vault = Vault::new("aws-s3", "ceo", "ceo");
    println!("{:?}", vault);
}