use serde::{Deserialize, Serialize};

use crate::base::{ComputeMode, InteractionPoint, Position, Rewrite, TokenBased};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct Status {
    show_implicit_arguments: bool,
    checked: bool,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct ResponseContextEntry {
    original_name: String,
    reified_name: String,
    binding: String,
    in_scope: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct CommandState {
    interaction_points: Vec<InteractionPoint>,
    current_file: String,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MakeCase {
    Function,
    ExtendedLambda,
}

#[serde(tag = "kind")]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum GoalType {
    GoalOnly,
    GoalAndHave { term: String },
    GoalAndElaboration { term: String },
}

#[serde(tag = "kind")]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum GoalInfo {
    HelperFunction {
        // TODO
    },
    NormalForm {
        #[serde(rename = "computeMode")]
        compute_mode: ComputeMode,
        expr: String,
    },
    GoalType {
        rewrite: Rewrite,
        #[serde(rename = "type")]
        goal_type: GoalType,
        entries: Vec<ResponseContextEntry>,
        #[serde(rename = "outputForms")]
        output_forms: (), // TODO
    },
    CurrentGoal {
        rewrite: Rewrite,
    },
    InferredType {
        expr: String,
    },
}

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
        #[serde(rename = "computeMode")]
        compute_mode: ComputeMode,
        #[serde(rename = "commandState")]
        command_state: CommandState,
        time: String,
        expr: String,
    },
    InferredType {
        #[serde(rename = "commandState")]
        command_state: CommandState,
        time: String,
        expr: String,
    },
    Context {
        #[serde(rename = "interactionPoint")]
        interaction_point: InteractionPoint,
        context: Vec<ResponseContextEntry>,
    },
    Version {
        version: String,
    },
    GoalSpecific {
        #[serde(rename = "interactionPoint")]
        interaction_point: InteractionPoint,
        #[serde(rename = "goalInfo")]
        goal_info: GoalInfo,
    },
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct AspectHighlight {
    range: (Position, Position),
    atoms: Vec<String>,
    token_based: TokenBased,
    note: Option<String>,
    definition_site: Option<DefinitionSite>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct DefinitionSite {
    filepath: String,
    position: Position,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct HighlightingInfo {
    remove: bool,
    payload: Vec<AspectHighlight>,
}

/// TODO: This enum is incomplete, contribution is welcomed.
#[serde(tag = "kind")]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum Resp {
    HighlightingInfo {
        info: Option<HighlightingInfo>,
        filepath: Option<String>,
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
