use crate::base::{ComputeMode, Rewrite};
use crate::pos::InteractionPoint;
use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct ResponseContextEntry {
    pub original_name: String,
    pub reified_name: String,
    pub binding: String,
    pub in_scope: String,
}

#[serde(tag = "kind")]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum GoalTypeAux {
    GoalOnly,
    GoalAndHave { expr: String },
    GoalAndElaboration { term: String },
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GoalType {
    pub rewrite: Rewrite,
    pub type_aux: GoalTypeAux,
    #[serde(rename = "type")]
    pub the_type: String,
    pub entries: Vec<ResponseContextEntry>,
    pub output_forms: Vec<String>,
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
    GoalType(GoalType),
    CurrentGoal {
        rewrite: Rewrite,
        #[serde(rename = "type")]
        the_type: String,
    },
    InferredType {
        expr: String,
    },
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GoalSpecific {
    pub interaction_point: InteractionPoint,
    pub goal_info: GoalInfo,
}
