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
