mod pentry;
mod prompt;
mod utils;

use crate::pentry::read_passwords_file;
use crate::pentry::Vault;
use crate::prompt::Prompt;
use crate::utils::println;

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

                let vault = Vault::new(service.as_str(), username.as_str(), password.as_str());

                println("ENTRY ADDED SUCCESSFULLY", Some(str_prefix_resp));
                let _ = vault.write_to_file().unwrap();
            }
            "2" => {
                todo!("Make the function async and return Result<> with Error and perform Error Handling for the returned Result")
                // let services = read_passwords_file();
            }
            "3" => {}
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
