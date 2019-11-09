use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum KindTag {
    HighlightingInfo,
    Status,
    JumpToError,
    InteractionPoints,
    GiveAction,
    /// Response is list of printed clauses.
    MakeCase,
    /// Solution for one or more meta-variables.
    SolveAll,
    DisplayInfo,
    /// The integer is the message's debug level.
    RunningInfo,
    ClearRunningInfo,
    /// Clear highlighting of the given kind.
    ClearHighlighting,
    /// A command sent when an abort command has completed successfully.
    DoneAborting,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Default, Debug, Eq, PartialEq, Hash)]
pub struct Status {
    show_implicit_arguments: bool,
    checked: bool,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum DisplayInfo {
    // TODO
}

pub type InteractionPoint = u32;

/// TODO: This enum is incomplete, contribution is welcomed.
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Resp {
    HighlightingInfo {
        kind: KindTag,
        filepath: String,
        direct: bool,
    },
    Status {
        kind: KindTag,
        status: Status,
    },
    JumpToError {
        kind: KindTag,
        // TODO
    },
    InteractionPoints {
        kind: KindTag,
        interaction_points: Vec<InteractionPoint>,
    },
    GiveAction {
        kind: KindTag,
        give_result: bool,
        interaction_point: InteractionPoint,
    },
    MakeCase {
        kind: KindTag,
        // TODO
    },
    SolveAll {
        kind: KindTag,
        // TODO
    },
    DisplayInfo {
        kind: KindTag,
        info: DisplayInfo,
    },
    RunningInfo {
        kind: KindTag,
        // TODO
    },
    ClearRunningInfo {
        kind: KindTag,
        // TODO
    },
    ClearHighlighting {
        kind: KindTag,
        // TODO
    },
    DoneAborting {
        kind: KindTag,
        // TODO
    },
}
