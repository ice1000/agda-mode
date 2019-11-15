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
        if line.is_empty() || line.chars().all(|c| c.is_whitespace()) {
            let cmds = UserInput::values().iter().map(|&s| s.to_owned()).collect();
            Ok((pos, cmds))
        } else {
            Ok((pos, Vec::default()))
        }
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
