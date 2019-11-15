use serde::{Deserialize, Serialize};

use crate::base::{ComputeMode, InteractionPoint, Position, Rewrite, TokenBased};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct Status {
    pub show_implicit_arguments: bool,
    pub checked: bool,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct ResponseContextEntry {
    pub original_name: String,
    pub reified_name: String,
    pub binding: String,
    pub in_scope: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct CommandState {
    pub interaction_points: Vec<InteractionPoint>,
    pub current_file: String,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MakeCase {
    Function,
    ExtendedLambda,
}

#[serde(tag = "kind")]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum GoalTypeAux {
    GoalOnly,
    GoalAndHave { expr: String },
    GoalAndElaboration { term: String },
}

/// Information about one goal.
#[serde(tag = "kind")]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum GoalInfo {
    HelperFunction {
        signature: String,
    },
    NormalForm {
        #[serde(rename = "computeMode")]
        compute_mode: ComputeMode,
        expr: String,
    },
    GoalType {
        rewrite: Rewrite,
        #[serde(rename = "typeAux")]
        type_aux: GoalTypeAux,
        #[serde(rename = "type")]
        the_type: String,
        entries: Vec<ResponseContextEntry>,
        #[serde(rename = "outputForms")]
        constraints: Vec<String>,
    },
    CurrentGoal {
        rewrite: Rewrite,
        #[serde(rename = "type")]
        the_type: String,
    },
    InferredType {
        expr: String,
    },
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
    AllGoalsWarnings {
        goals: (),
        warnings: String,
        errors: String,
    },
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

/// A token highlighting information.
/// The token is somehow called `Aspect` in Agda.
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct AspectHighlight {
    pub range: (Position, Position),
    pub atoms: Vec<String>,
    pub token_based: TokenBased,
    pub note: Option<String>,
    pub definition_site: Option<DefinitionSite>,
}

/// Jump to library definition information.
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct DefinitionSite {
    pub filepath: String,
    pub position: Position,
}

/// A list of token highlighting information.
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct HighlightingInfo {
    pub remove: bool,
    pub payload: Vec<AspectHighlight>,
}

/// Result of a "give" action.
///
/// The structure is very mysterious.
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq, Hash)]
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
        give_result: GiveResult,
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
