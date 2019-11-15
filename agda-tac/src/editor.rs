use rustyline::completion::Completer;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::{CompletionType, Config, Context, Editor, Helper};

use crate::input::UserInput;

pub struct CliEditor {
    // TODO
}

impl Completer for CliEditor {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let start = line
            .chars()
            .enumerate()
            .find(|(_, i)| !i.is_whitespace())
            .map(|(i, _)| i)
            .unwrap_or(0);
        let subs = if pos > start {
            &line[start..pos]
        } else {
            &line[start..]
        };
        let base = UserInput::values()
            .iter()
            .filter(|s| s.starts_with(subs))
            .map(|&s| s.to_owned())
            .collect();
        Ok((start, base))
    }
}

impl Hinter for CliEditor {
    // TODO
}

impl Highlighter for CliEditor {}

impl Helper for CliEditor {}

impl CliEditor {
    pub fn into_editor(self) -> Editor<CliEditor> {
        let mut r = Editor::with_config(
            Config::builder()
                .history_ignore_space(true)
                .completion_type(CompletionType::Circular)
                .build(),
        );
        r.set_helper(Some(self));
        r
    }
}
