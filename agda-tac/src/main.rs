use crate::repl::{repl, ReplState};
use agda_mode::base::{debug_command, debug_response};

mod args;
mod repl;

const FAIL: &str = "Failed to start Agda";
const FAIL_CMD: &str = "Failed to evaluate Agda command";

#[tokio::main]
async fn main() {
    let args = args::pre();
    unsafe {
        debug_command(args.debug_command);
        debug_response(args.debug_response);
    };
    let agda_program = args.agda.as_ref().map_or("agda", |s| &*s);
    let repl_state = ReplState::start(agda_program, args.file).await.expect(FAIL);
    repl(repl_state).await.expect(FAIL_CMD);
}
