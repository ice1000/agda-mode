use super::{GoalSpecific, InvisibleGoal, ResponseContextEntry, VisibleGoal};
use crate::base::ComputeMode;
use crate::pos::InteractionPoint;
use serde::Deserialize;

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct CommandState {
    pub interaction_points: Vec<InteractionPoint>,
    pub current_file: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct InferredType {
    pub command_state: CommandState,
    pub time: String,
    pub expr: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct NormalForm {
    pub compute_mode: ComputeMode,
    pub command_state: CommandState,
    pub time: String,
    pub expr: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Context {
    interaction_point: InteractionPoint,
    context: Vec<ResponseContextEntry>,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
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

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SearchResult {
    pub name: String,
    pub term: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AllGoalsWarnings {
    pub visible_goals: Vec<VisibleGoal>,
    pub invisible_goals: Vec<InvisibleGoal>,
    pub warnings: String,
    pub errors: String,
}

/// Something that is displayed in the Emacs mode,
/// serialized with more details.
#[serde(tag = "kind")]
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
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
    Error(AgdaError),
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
        results: Vec<SearchResult>,
    },
    WhyInScope {
        // TODO
    },
    NormalForm(NormalForm),
    InferredType(InferredType),
    Context(Context),
    Version {
        version: String,
    },
    GoalSpecific(GoalSpecific),
}
