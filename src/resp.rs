use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Default, Debug, Eq, PartialEq, Hash)]
pub struct Status {
    show_implicit_arguments: bool,
    checked: bool,
}

#[serde(tag = "kind")]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum DisplayInfo {
    // TODO
}

pub type InteractionPoint = u32;

/// TODO: This enum is incomplete, contribution is welcomed.
#[serde(tag = "kind")]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Resp {
    HighlightingInfo {
        filepath: String,
        direct: bool,
    },
    Status {
        status: Status,
    },
    JumpToError {
        // TODO
    },
    InteractionPoints {
        #[serde(rename = "interactionPoints")]
        interaction_points: Vec<InteractionPoint>,
    },
    GiveAction {
        #[serde(rename = "giveResult")]
        give_result: bool,
        #[serde(rename = "interactionPoint")]
        interaction_point: InteractionPoint,
    },
    /// Response is list of printed clauses.
    MakeCase {
        // TODO
    },
    /// Solution for one or more meta-variables.
    SolveAll {
        // TODO
    },
    DisplayInfo {
        info: DisplayInfo,
    },
    /// The integer is the message's debug level.
    RunningInfo {
        // TODO
    },
    ClearRunningInfo {
        // TODO
    },
    /// Clear highlighting of the given kind.
    ClearHighlighting {
        // TODO
    },
    /// A command sent when an abort command has completed successfully.
    DoneAborting {
        // TODO
    },
}
