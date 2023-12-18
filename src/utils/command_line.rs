use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::stdout;

#[derive(PartialEq, Debug, strum_macros::Display)]
pub enum PrintCommand {
    Success,
    Info,
    Error,
}

impl PrintCommand {
    pub fn print_message(&self, statement: &String) {
        let mut stdout: std::io::Stdout = stdout();

        // Decide on the print color
        let statment_color: Color = match self {
            Self::Success => Color::Green,
            Self::Info => Color::Yellow,
            Self::Error => Color::Red,
        };

        // Print the agine statement
        stdout.execute(SetForegroundColor(statment_color)).unwrap();
        print!("[{}] ", self);

        stdout.execute(ResetColor).unwrap();
        // Make selected color
        println!("{}", statement);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_prints_agent_msg() {
        PrintCommand::Success.print_message(&"I am a success.".to_string());
        PrintCommand::Info.print_message(&"I am a info.".to_string());
        PrintCommand::Error.print_message(&"I am an error.".to_string());
    }
}
