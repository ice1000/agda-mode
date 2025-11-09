use super::{GoalSpecific, InvisibleGoal, ResponseContextEntry, VisibleGoal};
use crate::base::{Cohesion, ComputeMode, Hiding, Relevance};
use crate::pos::InteractionPoint;
use crate::resp::OutputForm;
use serde::Deserialize;

#[derive(Deserialize, Clone, Default, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CommandState {
    pub interaction_points: Vec<InteractionPoint>,
    pub current_file: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InferredType {
    pub command_state: CommandState,
    pub time: String,
    pub expr: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NormalForm {
    pub compute_mode: ComputeMode,
    pub command_state: CommandState,
    pub time: String,
    pub expr: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub interaction_point: InteractionPoint,
    pub context: Vec<ResponseContextEntry>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgdaError {
    pub message: Option<String>,
}

impl Into<String> for AgdaError {
    fn into(self) -> String {
        self.message.unwrap_or_else(|| "Unknown error".to_owned())
    }
}

impl<Ok> Into<Result<Ok, String>> for AgdaError {
    fn into(self) -> Result<Ok, String> {
        Err(self.into())
    }
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NamedPrettyTCM {
    pub name: String,
    pub term: String,
}

/// One item in the `telToList` telescope list.
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TelescopicItem {
    pub dom: String,
    pub name: Option<String>,
    pub finite: bool,
    pub cohesion: Cohesion,
    pub relevance: Relevance,
    pub hiding: Hiding,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModuleContents {
    pub names: Vec<String>,
    pub contents: Vec<NamedPrettyTCM>,
    pub telescope: Vec<TelescopicItem>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AllGoalsWarnings {
    pub visible_goals: Vec<VisibleGoal>,
    pub invisible_goals: Vec<InvisibleGoal>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

/// Something that is displayed in the Emacs mode,
/// serialized with more details.
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "kind")]
pub enum DisplayInfo {
    CompilationOk {
        warnings: String,
        errors: String,
    },
    Constraints {
        constraints: Vec<OutputForm>,
    },
    AllGoalsWarnings(AllGoalsWarnings),
    Time {
        time: String,
    },
    Error(AgdaError),
    IntroNotFound,
    IntroConstructorUnknown {
        /// Available constructors
        constructors: Vec<String>,
    },
    Auto {
        info: String,
    },
    ModuleContents(ModuleContents),
    SearchAbout {
        search: String,
        results: Vec<NamedPrettyTCM>,
    },
    WhyInScope {
        thing: String,
        filepath: String,
        message: String,
    },
    NormalForm(NormalForm),
    InferredType(InferredType),
    Context(Context),
    Version {
        version: String,
    },
    GoalSpecific(GoalSpecific),
}
