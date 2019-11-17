use serde::{Deserialize, Serialize};

use crate::base::{ComputeMode, InteractionPoint};

pub use self::goal::*;
pub use self::hl::*;
pub use self::oc::*;

/// Goal information.
mod goal;
/// Highlighting.
mod hl;
/// Output constraints (user goals & unsolved metas).
mod oc;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct Status {
    pub show_implicit_arguments: bool,
    pub checked: bool,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct CommandState {
    pub interaction_points: Vec<InteractionPoint>,
    pub current_file: String,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MakeCaseVariant {
    Function,
    ExtendedLambda,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct InferredType {
    command_state: CommandState,
    time: String,
    expr: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct NormalForm {
    pub compute_mode: ComputeMode,
    pub command_state: CommandState,
    pub time: String,
    pub expr: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AllGoalsWarnings {
    pub visible_goals: Vec<VisibleGoal>,
    pub invisible_goals: Vec<InvisibleGoal>,
    pub warnings: String,
    pub errors: String,
}

/// Something that is displayed in the Emacs mode,
/// serialized with more details.
#[serde(tag = "kind")]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum DisplayInfo {
    CompilationOk {
        warnings: String,
        errors: String,
    },
    Constraints {
        // TODO
    },
    AllGoalsWarnings(AllGoalsWarnings),
    Time {
        time: String,
    },
    Error {
        message: Option<String>,
    },
    IntroNotFound {
        // TODO
    },
    IntroConstructorUnknown {
        // TODO
    },
    Auto {
        info: String,
    },
    ModuleContents {
        // TODO
    },
    SearchAbout {
        search: String,
        // TODO
    },
    WhyInScope {
        // TODO
    },
    NormalForm(NormalForm),
    InferredType(InferredType),
    Context {
        #[serde(rename = "interactionPoint")]
        interaction_point: InteractionPoint,
        context: Vec<ResponseContextEntry>,
    },
    Version {
        version: String,
    },
    GoalSpecific(GoalSpecific),
}

/// Result of a "give" action.
///
/// The structure is very mysterious.
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct GiveResult {
    pub str: Option<String>,
    pub paren: Option<bool>,
}

impl GiveResult {
    /// The return value is not actually a result.
    /// I just want an `Either` type.
    pub fn into_either(self) -> Result<String, bool> {
        match (self.str, self.paren) {
            (Some(s), None) => Ok(s),
            (None, Some(b)) => Err(b),
            _ => unreachable!(),
        }
    }
}

/// Agda response.
///
/// TODO: This enum is incomplete, contribution is welcomed.
#[serde(tag = "kind")]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum Resp {
    HighlightingInfo(HighlightingInfo),
    Status {
        status: Status,
    },
    JumpToError {
        filepath: String,
        position: i32,
    },
    InteractionPoints {
        #[serde(rename = "interactionPoints")]
        interaction_points: Vec<InteractionPoint>,
    },
    GiveAction {
        #[serde(rename = "giveResult")]
        give_result: GiveResult,
        #[serde(rename = "interactionPoint")]
        interaction_point: InteractionPoint,
    },
    /// Response is list of printed clauses.
    MakeCase {
        variant: MakeCaseVariant,
        #[serde(rename = "interactionPoint")]
        interaction_point: InteractionPoint,
        clauses: Vec<String>,
    },
    /// Solution for one or more meta-variables.
    SolveAll {
        // TODO
    },
    DisplayInfo {
        info: Option<DisplayInfo>,
    },
    /// The integer is the message's debug level.
    RunningInfo {
        #[serde(rename = "debugLevel")]
        debug_level: i32,
        message: String,
    },
    ClearRunningInfo,
    /// Clear highlighting of the given kind.
    ClearHighlighting {
        // TODO
    },
    /// A command sent when an abort command has completed successfully.
    DoneAborting,
}
