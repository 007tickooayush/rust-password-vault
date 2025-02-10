use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, Error, ErrorKind, Write};
use serde::{Deserialize, Serialize};
use crate::utils::println;

#[derive(Debug, Serialize, Deserialize)]
pub struct Vault {
    pub service: String,
    pub username: String,
    pub password: String,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Vault {
//     pub service: String,
//     pub username: String,
//     pub password: String
// }


impl Vault {
    pub fn new(service: String, username: String, password: String) -> Self {
        Vault {
            service,
            username,
            password
        }
    }

    pub fn from_json(json_string: String) -> Result<Self, serde_json::Error> {
        // todo!("Implement this function");
        // unimplemented!("Implement this function")
        serde_json::from_str(json_string.as_str())
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




#[test]
fn test_new_vault() {
    let vault = Vault::new(String::from("aws-s3"), String::from("ceo"), String::from("ceo"));
    println!("{:?}", vault);
}