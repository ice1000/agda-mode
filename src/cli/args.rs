use clap::{App, AppSettings};
use minitt_util::cli::{cli_completion_generation, GenShellSubCommand};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    about,
    name = "agda-tac",
    global_settings(&[AppSettings::ColoredHelp])
)]
pub struct CliOptions {
    /// The input file to type-check (Notice: file should be UTF-8 encoded)
    #[structopt(name = "FILE")]
    pub file: String,

    /// Path to your agda executable
    #[structopt(long, name = "path")]
    pub agda: Option<String>,

    #[structopt(subcommand)]
    completion: Option<GenShellSubCommand>,
}

fn app<'a, 'b>() -> App<'a, 'b> {
    let extra_help = "For extra help please head to \
                      https://github.com/ice1000/agda-mode-rs/issues/new";
    // Introduced a variable because stupid CLion :(
    let app: App = CliOptions::clap();
    app.after_help(extra_help)
}

pub fn pre() -> CliOptions {
    let args: CliOptions = CliOptions::from_clap(&app().get_matches());
    cli_completion_generation(&args.completion, app);
    args
}
