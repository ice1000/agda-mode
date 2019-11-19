use serde::Deserialize;

use crate::pos::InteractionPoint;

pub use self::di::*;
pub use self::give::*;
pub use self::goal::*;
pub use self::hl::*;
pub use self::oc::*;
use crate::base::TokenBased;

/// Display info.
mod di;
/// About the "Give" action.
mod give;
/// Goal information.
mod goal;
/// Highlighting.
mod hl;
/// Output constraints (user goals & unsolved metas).
mod oc;

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct MakeCase {
    pub variant: MakeCaseVariant,
    pub interaction_point: InteractionPoint,
    pub clauses: Vec<String>,
}

/// Status information.
#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct Status {
    /// Are implicit arguments displayed?
    pub show_implicit_arguments: bool,
    /// Has the module been successfully type checked?
    pub checked: bool,
}

#[derive(Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MakeCaseVariant {
    Function,
    ExtendedLambda,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct OneSolution {
    pub interaction_point: InteractionPoint,
    pub expression: String,
}

/// Agda response.
#[serde(tag = "kind")]
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
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
    GiveAction(GiveAction),
    /// Response is list of printed clauses.
    MakeCase(MakeCase),
    /// Solution for one or more meta-variables.
    SolveAll {
        solutions: Vec<OneSolution>,
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
        #[serde(rename = "tokenBased")]
        token_based: TokenBased,
    },
    /// A command sent when an abort command has completed successfully.
    DoneAborting,
}
