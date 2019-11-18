use agda_mode::base::InteractionPoint;

/// Parsed user input.
#[derive(Debug, Clone, Copy)]
pub enum UserInput<'a> {
    Define(&'a str),
    RawLine(&'a str),
    Give(InteractionPoint, &'a str),
    Reload,
    Help,
    Exit,
    Infer(InteractionPoint, &'a str),
    Simplify(InteractionPoint, &'a str),
    Normalize(InteractionPoint, &'a str),
    Type(InteractionPoint),
    Unknown(Option<&'a str>),
}

static VALUES: &[&str] = &[
    "help", "define", "line", "fill", "give", "reload", "infer", "simpl", "norm", "deduce", "type",
    "exit", "quit",
];

impl<'a> UserInput<'a> {
    pub fn values() -> &'static [&'static str] {
        VALUES
    }
}

impl<'a> UserInput<'a> {
    fn trim_and_parse_to_ip_str(
        line: &'a str,
        cmd: &str,
        alias: &str,
        ok: impl FnOnce(InteractionPoint, &'a str) -> Self,
    ) -> Self {
        let s = line
            .trim_start_matches(cmd)
            .trim_start_matches(alias)
            .trim_start();
        match s.find(" ") {
            None => UserInput::Unknown(Some("please specify a goal.")),
            Some(idx) => match s[..idx].trim().parse::<InteractionPoint>() {
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
        } else if line.starts_with("line") {
            UserInput::RawLine(line.trim_start_matches("line").trim_start())
        } else if line.starts_with("type") {
            match line
                .trim_start_matches("type")
                .trim()
                .parse::<InteractionPoint>()
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
        } else if line == "exit" || line == "quit" {
            UserInput::Exit
        } else {
            UserInput::Unknown(None)
        }
    }
}
