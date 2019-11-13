use tokio::io::BufReader;

use crate::repl::repl;
use agda_mode::agda::{start_agda, ProcessStdio};

mod args;
mod repl;

const FAIL: &str = "Failed to evaluate Agda command";
const START_FAIL: &str = "Failed to start Agda";

#[tokio::main]
async fn main() {
    let args = args::pre();
    let agda_program = args.agda.as_ref().map_or("agda", |s| &*s);
    let ProcessStdio(process, stdin, stdout) = start_agda(agda_program).expect(START_FAIL);
    tokio::spawn(async {
        let status = process.await.expect(START_FAIL);
        println!("Agda exits with status {}.", status);
    });
    repl(stdin, BufReader::new(stdout), args.file)
        .await
        .expect(FAIL);
}
