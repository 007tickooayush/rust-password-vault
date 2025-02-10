mod pentry;
mod prompt;
mod utils;

use crate::pentry::Vault;
use crate::prompt::Prompt;
use crate::utils::{println, read_passwords_file, search_passwords_file};

fn clr() {
    print!("{}[2J", 27 as char);
}

fn main() {
    clr();

    let mut action_counter = 0;

    let str_prefix_header = "+-+-+-+-+-+-+-";
    let str_prefix_choice = ">>>>>>>>>>>";
    let str_prefix_resp = "===========>";
    let str_prefix_footer = "X-X-X-X-X-X-X-X-X-X-X-X-X-X";

    let ascii = r#"
____    ____  ___       __       __    __  .___________. _______ .______
\   \  /   / /   \     |  |     |  |  |  | |           ||   ____||   _  \
 \   \/   / /  ^  \    |  |     |  |  |  | `---|  |----`|  |__   |  |_)  |
  \      / /  /_\  \   |  |     |  |  |  |     |  |     |   __|  |      /
   \    / /  _____  \  |  `----.|  `--'  |     |  |     |  |____ |  |\  \----.
    \__/ /__/     \__\ |_______| \______/      |__|     |_______|| _| `._____|

    "#;

    println!("{ascii}");

    loop {
        println("AVAILABLE OPTIONS:", Some(str_prefix_header));
        println("1. Add a new password", Some(str_prefix_choice));
        println("2. View all passwords", Some(str_prefix_choice));
        println("3. Search for a password", Some(str_prefix_choice));
        println("4. Exit", Some(str_prefix_choice));

        let mut choice = String::new();

        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();

                let service = Prompt::prompt(
                    &String::from("ENTER SERVICE NAME    :"),
                    Some(str_prefix_choice),
                )
                .unwrap();
                let username = Prompt::prompt(
                    &mut String::from("ENTER USERNAME    :"),
                    Some(str_prefix_choice),
                )
                .unwrap();
                let password = Prompt::prompt(
                    &mut String::from("ENTER PASSWORD    :"),
                    Some(str_prefix_choice),
                )
                .unwrap();

                let vault = Vault::new(service, username, password);

                println("ENTRY ADDED SUCCESSFULLY", Some(str_prefix_resp));
                let _ = vault.write_to_file().unwrap();
            }
            "2" => {
                // todo!("Make the function async and return Result<> with Error and perform Error Handling for the returned Result")
                let services = read_passwords_file().map_err(|e| {
                    println(&e.to_string(), None);
                    e
                });

                match services {
                    Ok(services) => {
                        for service in services {
                            println(service.service.as_str(), Some("SERVICE: "));
                            println(service.username.as_str(), Some("USERNAME: "));
                            println(service.password.as_str(), Some("PASSWORD: "));
                            println!("\n\n");
                        }
                    }
                    Err(e) => {
                        println(&e.to_string(), None);
                    }
                }

            }
            "3" => {
                let search_term = Prompt::prompt(
                    "PROVIDE SEARCH TERM: ",
                    Some(str_prefix_choice),
                );

                if let Ok(search_term) = search_term {
                    if !search_term.is_empty() {
                        // improvised search technique
                        // typecasting and rendering only the line that contains the search results

                        let search_results = search_passwords_file(&search_term).unwrap();

                        println!("\n\n");
                        for item in search_results {
                            println(item.service.as_str(), Some("SERVICE: "));
                            println(item.username.as_str(), Some("USERNAME: "));
                            println(item.password.as_str(), Some("PASSWORD: "));
                            println!("\n\n");
                        }

                    }
                }

            }
            "4" => {
                break;
            }
            _ => {
                println(" INVALID CHOICE.", Some(str_prefix_resp));
            }
        };

        action_counter += 1;
        println(
            format!(" END of response: {action_counter} {str_prefix_footer}\n\n").as_str(),
            Some(str_prefix_footer),
        );
    }
}
