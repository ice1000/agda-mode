use rustyline::completion::Completer;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::{CompletionType, Config, Editor, Helper};

pub struct CliEditor {
    // TODO
}

impl Completer for CliEditor {
    // TODO
    type Candidate = String;
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
