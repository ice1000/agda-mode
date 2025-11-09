use crate::base::{ComputeMode, Rewrite};
use crate::pos::InteractionPoint;
use serde::Deserialize;

#[derive(Deserialize, Clone, Default, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResponseContextEntry {
    pub original_name: String,
    pub reified_name: String,
    pub binding: String,
    pub in_scope: bool,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "kind")]
pub enum GoalTypeAux {
    GoalOnly,
    GoalAndHave { expr: String },
    GoalAndElaboration { term: String },
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GoalType {
    pub rewrite: Rewrite,
    pub type_aux: GoalTypeAux,
    pub r#type: String,
    pub entries: Vec<ResponseContextEntry>,
    pub boundary: Vec<String>,
    pub output_forms: Vec<String>,
}

/// Information about one goal.
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "kind")]
pub enum GoalInfo {
    HelperFunction {
        signature: String,
    },
    NormalForm {
        #[serde(rename = "computeMode")]
        compute_mode: ComputeMode,
        expr: String,
    },
    GoalType(GoalType),
    CurrentGoal {
        rewrite: Rewrite,
        r#type: String,
    },
    InferredType {
        expr: String,
    },
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GoalSpecific {
    pub interaction_point: InteractionPoint,
    pub goal_info: GoalInfo,
}
