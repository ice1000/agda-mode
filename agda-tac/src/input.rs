use agda_mode::pos::InteractionId;

/// Parsed user input.
#[derive(Debug, Clone, Copy)]
pub enum UserInput<'a> {
    Define(&'a str),
    RawLine(&'a str),
    Give(InteractionId, &'a str),
    Reload,
    Help,
    ListGoals,
    SearchModule(&'a str),
    Exit,
    Infer(InteractionId, &'a str),
    Simplify(InteractionId, &'a str),
    Normalize(InteractionId, &'a str),
    Type(InteractionId),
    Unknown(Option<&'a str>),
    ToggleDebugCommand,
    ToggleDebugResponse,
}

static VALUES: &[&str] = &[
    "help",
    "define",
    "line-raw",
    "fill",
    "give",
    "reload",
    "list-goals",
    "infer",
    "simpl",
    "norm",
    "deduce",
    "type",
    "exit",
    "quit",
    "debug-response",
    "debug-command",
];

pub static HELP: &[&str] = &[
    "help: print this message.",
    "define <name>: define a function, with the given `name`.",
    "line-raw <line>: append a `line` to the agda file, with leading whitespaces preserved.",
    "list-goals: list the goals and their line number",
    "fill <goal> <code>: fill the `goal` with `code` (alias: give).",
    "infer <goal> <code>: infer the type of `code` under the context of `goal` (alias: deduce).",
    "norm <goal> <code>: normalize `code` in `goal` (alias: simpl).",
    "type <goal>: show the type of the `goal`.",
    "exit: exit the REPL (alias: quit).",
];

impl<'a> UserInput<'a> {
    pub fn values() -> &'static [&'static str] {
        VALUES
    }

    fn trim_and_parse_to_ip_str(
        line: &'a str,
        cmd: &str,
        alias: &str,
        ok: impl FnOnce(InteractionId, &'a str) -> Self,
    ) -> Self {
        let s = line
            .trim_start_matches(cmd)
            .trim_start_matches(alias)
            .trim_start();
        match s.find(" ") {
            None => UserInput::Unknown(Some("please specify a goal.")),
            Some(idx) => match s[..idx].trim().parse::<InteractionId>() {
                Ok(i) => ok(i, s[idx..].trim()),
                Err(_) => UserInput::Unknown(Some("I cannot parse the goal number.")),
            },
        }
    }
}

impl<'a> From<&'a str> for UserInput<'a> {
    fn from(line: &'a str) -> Self {
        if line == "help" {
            UserInput::Help
        } else if line.starts_with("define") {
            UserInput::Define(line.trim_start_matches("define").trim_start())
        } else if line.starts_with("line-raw ") {
            UserInput::RawLine(line.trim_start_matches("line-raw "))
        } else if line.starts_with("type") {
            match line
                .trim_start_matches("type")
                .trim()
                .parse::<InteractionId>()
            {
                Ok(i) => UserInput::Type(i),
                Err(_) => UserInput::Unknown(Some("I cannot parse the goal number.")),
            }
        } else if line.starts_with("fill") || line.starts_with("give") {
            Self::trim_and_parse_to_ip_str(line, "fill", "give", UserInput::Give)
        } else if line.starts_with("infer") || line.starts_with("deduce") {
            Self::trim_and_parse_to_ip_str(line, "infer", "deduce", UserInput::Infer)
        } else if line.starts_with("simpl") {
            Self::trim_and_parse_to_ip_str(line, "simpl", "", UserInput::Simplify)
        } else if line.starts_with("norm") {
            Self::trim_and_parse_to_ip_str(line, "norm", "", UserInput::Normalize)
        } else if line == "reload" {
            UserInput::Reload
        } else if line == "list-goals" {
            UserInput::ListGoals
        } else if line == "list-module-contents" {
            UserInput::SearchModule(line.trim_start_matches("list-module-contents").trim())
        } else if line == "exit" || line == "quit" {
            UserInput::Exit
        } else if line == "debug-response" {
            UserInput::ToggleDebugResponse
        } else if line == "debug-command" {
            UserInput::ToggleDebugCommand
        } else {
            UserInput::Unknown(None)
        }
    }
}
