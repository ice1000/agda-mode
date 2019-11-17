use crate::base::{Comparison, InteractionPoint, Polarity};
use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct FindInstanceCandidate {
    #[serde(rename = "type")]
    pub of_type: String,
    pub value: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct JustSomething<Obj> {
    pub constraint_obj: Obj,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct PostponedCheckArgs<Obj> {
    pub constraint_obj: Obj,
    pub of_type: String,
    #[serde(rename = "type")]
    pub the_type: String,
    pub arguments: Vec<String>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CmpSomething<Obj> {
    pub constraint_objs: (Obj, Obj),
    pub comparison: Comparison,
}

#[serde(tag = "kind")]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum OutputConstraint<Obj> {
    OfType {
        #[serde(rename = "constraintObj")]
        constraint_obj: Obj,
        #[serde(rename = "type")]
        of_type: String,
    },
    CmpInType {
        #[serde(rename = "constraintObjs")]
        constraint_objs: (Obj, Obj),
        #[serde(rename = "type")]
        of_type: String,
        comparison: Comparison,
    },
    CmpElim {
        #[serde(rename = "constraintObjs")]
        constraint_objs: (Vec<Obj>, Vec<Obj>),
        #[serde(rename = "type")]
        of_type: String,
        polarities: Vec<Polarity>,
    },
    JustType(JustSomething<Obj>),
    JustSort(JustSomething<Obj>),
    CmpTypes(CmpSomething<Obj>),
    CmpLevels(CmpSomething<Obj>),
    CmpTeles(CmpSomething<Obj>),
    CmpSorts(CmpSomething<Obj>),
    Guard {
        constraint_objs: Box<OutputConstraint<Obj>>,
        problem: String,
    },
    Assign {
        #[serde(rename = "constraintObj")]
        constraint_obj: Obj,
        value: String,
    },
    TypedAssign {
        #[serde(rename = "constraintObj")]
        constraint_obj: Obj,
        #[serde(rename = "type")]
        of_type: String,
        value: String,
    },
    PostponedCheckArgs(PostponedCheckArgs<Obj>),
    IsEmptyType {
        #[serde(rename = "type")]
        the_type: String,
    },
    SizeLtSat {
        #[serde(rename = "type")]
        the_type: String,
    },
    FindInstanceOF {
        #[serde(rename = "constraintObj")]
        constraint_obj: Obj,
        #[serde(rename = "type")]
        of_type: String,
        candidates: Vec<FindInstanceCandidate>,
    },
    PTSInstance {
        #[serde(rename = "constraintObjs")]
        constraint_objs: (Obj, Obj),
    },
    PostponedCheckFunDef {
        name: String,
        #[serde(rename = "type")]
        of_type: String,
    },
}

pub type VisibleGoal = OutputConstraint<InteractionPoint>;
pub type InvisibleGoal = OutputConstraint<String>;
