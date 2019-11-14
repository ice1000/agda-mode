use crate::repl::{repl, ReplState};
use agda_mode::agda::start_agda;
use agda_mode::base::{debug_command, debug_response};
use tokio::io::BufReader;

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
    let (stdin, out) = start_agda(agda_program);
    let repl_state = ReplState::init(stdin, BufReader::new(out), args.file)
        .await
        .expect(FAIL);
    repl(repl_state).await.expect(FAIL_CMD);
}
