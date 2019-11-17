use serde::{Deserialize, Serialize};

use crate::base::InteractionPoint;

pub use self::di::*;
pub use self::give::*;
pub use self::goal::*;
pub use self::hl::*;
pub use self::oc::*;

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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct Status {
    pub show_implicit_arguments: bool,
    pub checked: bool,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MakeCaseVariant {
    Function,
    ExtendedLambda,
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
    GiveAction(GiveAction),
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
