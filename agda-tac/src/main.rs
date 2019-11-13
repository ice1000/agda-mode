use crate::repl::repl;
use agda_mode::agda::start_agda;
use tokio::io::BufReader;

mod args;
mod repl;

#[tokio::main]
async fn main() {
    let args = args::pre();
    let agda_program = args.agda.as_ref().map_or("agda", |s| &*s);
    let (stdin, out) = start_agda(agda_program).await;
    (repl(stdin, BufReader::new(out), args.file).await).expect("Failed to evaluate Agda command");
}
