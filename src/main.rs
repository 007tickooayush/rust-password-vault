mod pentry;

use crate::pentry::prompt;
use crate::pentry::read_passwords_file;


fn clr() {
    print!("{}[2J", 27 as char);
}

fn main() {
    clr();
    println!("Hello, world!");

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
        println!("Available Options:-");
        println!("1. Add a new password");
        println!("2. View all passwords");
        println!("3. Search for a password");
        println!("4. Exit");


        let mut choice = String::new();

        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {

            }
            "2" => {

            }
            "3" => {

            }
            "4" => {

            }
            _ => {

            }
        };
    }

}
