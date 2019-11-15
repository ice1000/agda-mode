use agda_mode::base::InteractionPoint;
use std::num::ParseIntError;

/// Parsed user input.
#[derive(Debug, Clone, Copy)]
pub enum UserInput<'a> {
    Define(&'a str),
    Give(InteractionPoint, &'a str),
    Reload,
    Help,
    Exit,
    Unknown(Option<&'a str>),
}

static VALUES: &[&str] = &["help", "define", "fill", "give", "reload", "exit", "quit"];

impl<'a> UserInput<'a> {
    pub fn values() -> &'static [&'static str] {
        VALUES
    }
}

impl<'a> From<&'a str> for UserInput<'a> {
    fn from(line: &'a str) -> Self {
        if line == "help" {
            UserInput::Help
        } else if line.starts_with("define") {
            UserInput::Define(line.trim_start_matches("define").trim_start())
        } else if line.starts_with("fill") || line.starts_with("give") {
            let s = line
                .trim_start_matches("fill")
                .trim_start_matches("give")
                .trim_start();
            match s.find(" ") {
                None => UserInput::Unknown(Some("please specify a goal.")),
                Some(idx) => match s[..idx].trim().parse::<InteractionPoint>() {
                    Ok(i) => UserInput::Give(i, s[idx..].trim()),
                    Err(_) => UserInput::Unknown(Some("I cannot parse the goal number.")),
                },
            }
        } else if line == "reload" {
            UserInput::Reload
        } else if line == "exit" || line == "quit" {
            UserInput::Exit
        } else {
            UserInput::Unknown(None)
        }
    }
}
