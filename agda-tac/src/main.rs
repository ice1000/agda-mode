use tokio::io::{AsyncWriteExt, BufReader, AsyncBufReadExt};

use agda_mode::agda::{load_file, start_agda};
use agda_mode::resp::Resp;

mod args;
#[cfg(test)]
mod tests;

const DES_FAIL: &str = "Failed to deserialize Agda response";
const EVAL_FAIL: &str = "Failed to evaluate Agda command";
const READ_FAIL: &str = "Failed to read from Agda";
const START_FAIL: &str = "Failed to start Agda";

#[tokio::main]
async fn main() {
    let args = args::pre();
    let file = args.file;
    let agda = start_agda(args.agda.as_ref().map_or("agda", |s| &*s)).expect(START_FAIL);
    let load_file = load_file(file);
    let mut stdin = agda.stdin;
    let mut stdout = BufReader::new(agda.stdout);
    let agda_process = agda.process;
    tokio::spawn(async {
        agda_process.await.expect(START_FAIL);
    });
    let mut buf = String::with_capacity(2045);
    stdin.write(load_file.to_string().as_bytes()).await.expect(EVAL_FAIL);
    stdin.flush().await.expect(EVAL_FAIL);
    stdout.read_line(&mut buf).await.expect(READ_FAIL);
    let resp: Resp = serde_json::from_str(&buf).expect(DES_FAIL);
    println!("{:?}", resp);
}
