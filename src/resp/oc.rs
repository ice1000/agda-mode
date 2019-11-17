use crate::base::{Comparison, InteractionPoint, Polarity};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Error, Formatter};

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
        constraint: Box<OutputConstraint<Obj>>,
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

pub trait CollectObjs<Obj> {
    fn collect_objs(&self, collect: impl FnMut(&Obj) + Copy);
}

impl<Obj> CollectObjs<Obj> for JustSomething<Obj> {
    fn collect_objs(&self, collect: impl FnMut(&Obj) + Copy) {
        collect(&self.constraint_obj)
    }
}

impl<Obj> CollectObjs<Obj> for Vec<Obj> {
    fn collect_objs(&self, collect: impl FnMut(&Obj) + Copy) {
        self.iter().for_each(collect)
    }
}

impl<Obj> CollectObjs<Obj> for (Obj, Obj) {
    fn collect_objs(&self, collect: impl FnMut<&Obj> + Copy) {
        let (a, b) = self;
        collect(a);
        collect(b);
    }
}

impl<Obj> CollectObjs<Obj> for CmpSomething<Obj> {
    fn collect_objs(&self, collect: impl FnMut(&Obj) + Copy) {
        self.constraint_objs.collect_objs(collect);
    }
}

impl<Obj> CollectObjs<Obj> for OutputConstraint<Obj> {
    fn collect_objs(&self, mut collect: impl FnMut(&Obj) + Copy) {
        use OutputConstraint::*;
        match self {
            OfType { constraint_obj, .. } => collect(constraint_obj),
            CmpInType {
                constraint_objs, ..
            } => constraint_objs.collect_objs(collect),
            CmpElim {
                constraint_objs: (xs, ys),
                ..
            } => {
                xs.collect_objs(collect);
                ys.collect_objs(collect);
            }
            JustType(a) => a.collect_objs(collect),
            JustSort(a) => a.collect_objs(collect),
            CmpTypes(c) => c.collect_objs(collect),
            CmpLevels(c) => c.collect_objs(collect),
            CmpTeles(c) => c.collect_objs(collect),
            CmpSorts(c) => c.collect_objs(collect),
            Guard { constraint, .. } => constraint.collect_objs(collect),
            Assign { constraint_obj, .. } => collect(constraint_obj),
            TypedAssign { constraint_obj, .. } => collect(constraint_obj),
            PostponedCheckArgs(o) => unimplemented!(),
            IsEmptyType { .. } => {}
            SizeLtSat { .. } => {}
            FindInstanceOF { constraint_obj, .. } => collect(constraint_obj),
            PTSInstance { constraint_objs } => constraint_objs.collect_objs(collect),
            PostponedCheckFunDef { .. } => {}
        }
    }
}

impl<Obj: Display> Display for OutputConstraint<Obj> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use OutputConstraint::*;
        unimplemented!()
    }
}
