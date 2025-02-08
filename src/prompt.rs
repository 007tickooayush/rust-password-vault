use std::io;
use std::io::Write;
use crate::utils::println;

pub struct Prompt<'inp> {
    pub message: &'inp str,
    pub prefix: Option<&'inp str>,
}

impl<'inp> Prompt<'inp> {
    /// Function to create prompt using input string
    /// NOTE: can not be tested as it requires user input from stdin
    ///
    pub fn prompt<'out>(prompt: &'inp str, prefix: Option<&'inp str>) -> Result<String, io::Error> {
        println(prompt, prefix);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        // .trim() removes the newline character from the input
        Ok(input.trim().to_string())
    }

}