use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Default, Debug, Eq, PartialEq, Hash)]
pub struct Status {
    show_implicit_arguments: bool,
    checked: bool,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
pub enum MakeCase {
    Function,
    ExtendedLambda,
}

#[serde(tag = "kind")]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum DisplayInfo {
    CompilationOk {
        warnings: String,
        errors: String,
    },
    Constraints {
        // TODO
    },
    AllGoalsWarnings {
        goals: (),
        warnings: String,
        errors: String,
    },
    Time {
        time: String,
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
    NormalForm {
        // TODO
    },
    InferredType {
        // TODO
    },
    Context {
        #[serde(rename = "interactionPoint")]
        interaction_point: InteractionPoint,
        // TODO
    },
    Version {
        version: String,
    },
    GoalSpecific {
        #[serde(rename = "interactionPoint")]
        interaction_point: InteractionPoint,
        // TODO
    },
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
        filepath: String,
        position: i32,
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
        variant: MakeCase,
        #[serde(rename = "interactionPoint")]
        interaction_point: InteractionPoint,
        clauses: Vec<String>,
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
