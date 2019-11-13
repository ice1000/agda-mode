use tokio::io::BufReader;

use agda_mode::agda::start_agda;

mod args;
mod repl;

const EVAL_FAIL: &str = "Failed to evaluate Agda command";
const START_FAIL: &str = "Failed to start Agda";

#[tokio::main]
async fn main() {
    let args = args::pre();
    let file = args.file;
    let agda = start_agda(args.agda.as_ref().map_or("agda", |s| &*s)).expect(START_FAIL);
    let stdin = agda.stdin;
    let stdout = BufReader::new(agda.stdout);
    let agda_process = agda.process;
    tokio::spawn(async {
        let status = agda_process.await.expect(START_FAIL);
        println!("Agda exits with status {}.", status);
    });
    repl::repl(stdin, stdout, file).await.expect(EVAL_FAIL);
}
