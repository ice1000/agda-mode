use std::io;

use tokio::io::{AsyncWriteExt, BufReader};
use tokio::net::process::{ChildStdin, ChildStdout};

use crate::cmd::{Cmd, IOTCM};
use crate::pos::InteractionPoint;
use crate::resp::{AgdaError, DisplayInfo, Resp};

use super::{load_file, send_command, start_agda, AgdaRead, JustStdio};

/// Simple REPL state wrapper.
pub struct ReplState {
    pub stdin: ChildStdin,
    pub agda: AgdaRead,
    pub file: String,
    interaction_points: Vec<InteractionPoint>,
    iotcm: IOTCM,
}

/// An Agda response that is either something good or some error.
pub type AgdaResult<T> = Result<T, String>;
/// Return type of `next_*` functions.
pub type NextResult<T> = io::Result<AgdaResult<T>>;

pub fn preprint_agda_result<T>(t: AgdaResult<T>) -> Option<T> {
    match t {
        Ok(o) => Some(o),
        Err(e) => {
            eprintln!("Errors:");
            eprintln!("{}", e);
            None
        }
    }
}

impl ReplState {
    pub async fn start(agda_program: &str, file: String) -> io::Result<Self> {
        let JustStdio(stdin, out) = start_agda(agda_program);
        Self::from_io(stdin, BufReader::new(out), file).await
    }

    /// Print all goals.
    pub fn print_goal_list(&self) {
        let ips = self.interaction_points();
        if ips.is_empty() {
            println!("No goals, you're all set.");
        }
        for interaction_point in ips {
            // This shouldn't fail
            let range = &interaction_point.range;
            debug_assert_eq!(range.len(), 1);
            let interval = &range[0];
            println!("?{} at line {}", interaction_point.id, interval.start.line)
        }
    }

    pub async fn from_io(
        mut stdin: ChildStdin,
        stdout: BufReader<ChildStdout>,
        file: String,
    ) -> io::Result<Self> {
        let iotcm = load_file(file.clone());
        send_command(&mut stdin, &iotcm).await?;
        Ok(Self {
            file,
            iotcm,
            stdin,
            interaction_points: vec![],
            agda: AgdaRead::from(stdout),
        })
    }

    pub async fn reload_file(&mut self) -> io::Result<()> {
        self.command(Cmd::load_simple(self.file.clone())).await
    }

    pub async fn command(&mut self, cmd: Cmd) -> io::Result<()> {
        self.iotcm.command = cmd;
        send_command(&mut self.stdin, &self.iotcm).await
    }

    pub async fn command_raw(&mut self, raw_command: &str) -> io::Result<()> {
        self.stdin.write(raw_command.as_bytes()).await?;
        self.stdin.flush().await
    }

    pub async fn shutdown(&mut self) -> io::Result<()> {
        self.stdin.shutdown().await
    }

    /// Await the next Agda response.
    pub async fn response(&mut self) -> io::Result<Resp> {
        self.agda.response().await
    }

    /// Skip information until the next display info.
    pub async fn next_display_info(&mut self) -> io::Result<DisplayInfo> {
        loop {
            match self.response().await? {
                Resp::DisplayInfo { info: Some(info) } => break Ok(info),
                _ => {}
            }
        }
    }

    /// Returns the latest [`next_goals`](Self::next_goals) result.
    pub fn interaction_points(&self) -> &[InteractionPoint] {
        &self.interaction_points
    }

    /// Skip information until the next interaction point (goal) list.
    /// The result can be queried via [`interaction_points`](Self::interaction_points).
    ///
    /// # Note
    ///
    /// This information normally comes right after `all_goals_warnings`,
    /// and when you call [`next_all_goals_warnings`](Self::next_all_goals_warnings),
    /// you've already eliminated errors.
    /// Therefore this method don't deal with errors.
    pub async fn next_goals(&mut self) -> io::Result<()> {
        use Resp::*;
        self.interaction_points = loop {
            match self.response().await? {
                InteractionPoints { interaction_points } => break interaction_points,
                _ => {}
            }
        };
        Ok(())
    }

    /// Skip information until an error.
    pub async fn next_error(&mut self) -> io::Result<AgdaError> {
        use crate::resp::DisplayInfo::Error as DisError;
        loop {
            match self.next_display_info().await? {
                DisError(e) => break Ok(e),
                _ => {}
            }
        }
    }
}

macro_rules! next_resp_of {
    ($f:ident, $p:ident, $d:literal) => {
        next_resp_of!($f, $p, crate::resp::$p, $d);
    };

    ($f:ident, $p:ident, $t:ty, $d:literal) => {
        impl ReplState {
            #[doc($d)]
            pub async fn $f(&mut self) -> NextResult<$t> {
                loop {
                    match self.response().await? {
                        Resp::$p(ga) => break Ok(Ok(ga)),
                        Resp::DisplayInfo {
                            info: Some(crate::resp::DisplayInfo::Error(e)),
                        } => break Ok(e.into()),
                        _ => {}
                    }
                }
            }
        }
    };
}

next_resp_of!(next_give_action, GiveAction, "Skip until next give-action.");
next_resp_of!(next_make_case, MakeCase, "Skip until next make-case.");
next_resp_of!(
    next_highlight,
    HighlightingInfo,
    "Skip until next highlight."
);

macro_rules! next_disp_of {
    ($f:ident, $p:ident, $d:literal) => {
        next_disp_of!($f, $p, crate::resp::$p, $d);
    };

    ($f:ident, $p:ident, $t:ty, $d:literal) => {
        impl ReplState {
            #[doc($d)]
            pub async fn $f(&mut self) -> NextResult<$t> {
                loop {
                    match self.next_display_info().await? {
                        DisplayInfo::Error(e) => break Ok(e.into()),
                        DisplayInfo::$p(agw) => break Ok(Ok(agw)),
                        _ => {}
                    }
                }
            }
        }
    };
}

next_disp_of!(
    next_all_goals_warnings,
    AllGoalsWarnings,
    "Skip until next interaction point (goal) list."
);
next_disp_of!(
    next_goal_specific,
    GoalSpecific,
    "Skip until next goal specific information."
);
next_disp_of!(
    next_module_contents,
    ModuleContents,
    "Skip until next module contents response."
);
next_disp_of!(next_normal_form, NormalForm, "Skip until next normal form.");
next_disp_of!(next_context, Context, "Skip until next context.");
next_disp_of!(
    next_inferred_type,
    InferredType,
    "Skip until next inferred type."
);
