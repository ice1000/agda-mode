/// Parsed user input.
#[derive(Debug, Clone, Copy)]
pub enum UserInput<'a> {
    Define(&'a str),
    Reload,
    Help,
    Exit,
    Unknown,
}

impl<'a> From<&'a str> for UserInput<'a> {
    fn from(line: &'a str) -> Self {
        if line == "help" {
            UserInput::Help
        } else if line.starts_with("define") {
            UserInput::Define(line.trim_start_matches("define").trim_start())
        } else if line == "reload" {
            UserInput::Reload
        } else if line == "exit" || line == "quit" {
            UserInput::Exit
        } else {
            UserInput::Unknown
        }
    }
}
