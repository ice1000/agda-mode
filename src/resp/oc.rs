use crate::base::{Comparison, Polarity};
use crate::pos::InteractionPoint;
use serde::Deserialize;
use std::fmt::{Display, Error, Formatter};

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct FindInstanceCandidate {
    #[serde(rename = "type")]
    pub of_type: String,
    pub value: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct JustSomething<Obj> {
    pub constraint_obj: Obj,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct PostponedCheckArgs<Obj> {
    pub constraint_obj: Obj,
    pub of_type: String,
    #[serde(rename = "type")]
    pub the_type: String,
    pub arguments: Vec<String>,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CmpSomething<Obj> {
    pub constraint_objs: (Obj, Obj),
    pub comparison: Comparison,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct FindInstanceOF<Obj> {
    pub constraint_obj: Obj,
    #[serde(rename = "type")]
    pub of_type: String,
    pub candidates: Vec<FindInstanceCandidate>,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TypedAssign<Obj> {
    pub constraint_obj: Obj,
    #[serde(rename = "type")]
    pub of_type: String,
    pub value: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct OfType<Obj> {
    pub constraint_obj: Obj,
    #[serde(rename = "type")]
    pub of_type: String,
}

#[serde(tag = "kind")]
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum OutputConstraint<Obj> {
    OfType(OfType<Obj>),
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
    TypedAssign(TypedAssign<Obj>),
    PostponedCheckArgs(PostponedCheckArgs<Obj>),
    IsEmptyType {
        #[serde(rename = "type")]
        the_type: String,
    },
    SizeLtSat {
        #[serde(rename = "type")]
        the_type: String,
    },
    FindInstanceOF(FindInstanceOF<Obj>),
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

impl<Obj> OutputConstraint<Obj> {
    pub fn try_into_of_type(self) -> Result<OfType<Obj>, Self> {
        if let OutputConstraint::OfType(o) = self {
            Ok(o)
        } else {
            Err(self)
        }
    }

    pub fn try_as_of_type(&self) -> Result<&OfType<Obj>, &Self> {
        if let OutputConstraint::OfType(o) = self {
            Ok(o)
        } else {
            Err(self)
        }
    }
}

pub trait CollectObjs<Obj> {
    fn collect_objs(&self, collect: impl FnMut(&Obj));
}

macro_rules! simple_collect_objs {
    ($class:ident) => {
        impl<Obj> CollectObjs<Obj> for $class<Obj> {
            fn collect_objs(&self, mut collect: impl FnMut(&Obj)) {
                collect(&self.constraint_obj)
            }
        }
    };
}

simple_collect_objs!(JustSomething);
simple_collect_objs!(PostponedCheckArgs);
simple_collect_objs!(OfType);
simple_collect_objs!(FindInstanceOF);
simple_collect_objs!(TypedAssign);

impl<Obj> CollectObjs<Obj> for Vec<Obj> {
    fn collect_objs(&self, collect: impl FnMut(&Obj)) {
        self.iter().for_each(collect)
    }
}

impl<Obj> CollectObjs<Obj> for (Obj, Obj) {
    fn collect_objs(&self, mut collect: impl FnMut(&Obj)) {
        let (a, b) = self;
        collect(a);
        collect(b);
    }
}

impl<Obj> CollectObjs<Obj> for (Vec<Obj>, Vec<Obj>) {
    fn collect_objs(&self, mut collect: impl FnMut(&Obj)) {
        let (a, b) = self;
        a.collect_objs(|x| collect(x));
        b.collect_objs(collect);
    }
}

impl<Obj> CollectObjs<Obj> for CmpSomething<Obj> {
    fn collect_objs(&self, collect: impl FnMut(&Obj)) {
        self.constraint_objs.collect_objs(collect);
    }
}

impl<Obj> CollectObjs<Obj> for OutputConstraint<Obj> {
    fn collect_objs(&self, mut collect: impl FnMut(&Obj)) {
        use OutputConstraint::*;
        match self {
            OfType(o) => o.collect_objs(collect),
            CmpInType {
                constraint_objs, ..
            } => constraint_objs.collect_objs(collect),
            CmpElim {
                constraint_objs, ..
            } => constraint_objs.collect_objs(collect),
            JustType(a) => a.collect_objs(collect),
            JustSort(a) => a.collect_objs(collect),
            CmpTypes(c) => c.collect_objs(collect),
            CmpLevels(c) => c.collect_objs(collect),
            CmpTeles(c) => c.collect_objs(collect),
            CmpSorts(c) => c.collect_objs(collect),
            Guard { constraint, .. } => constraint.collect_objs(collect),
            Assign { constraint_obj, .. } => collect(constraint_obj),
            TypedAssign(o) => o.collect_objs(collect),
            PostponedCheckArgs(o) => o.collect_objs(collect),
            IsEmptyType { .. } => {}
            SizeLtSat { .. } => {}
            FindInstanceOF(o) => o.collect_objs(collect),
            PTSInstance { constraint_objs } => constraint_objs.collect_objs(collect),
            PostponedCheckFunDef { .. } => {}
        }
    }
}

impl<Obj: Display> Display for JustSomething<Obj> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.constraint_obj.fmt(f)
    }
}

impl<Obj: Display> Display for CmpSomething<Obj> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let (a, b) = &self.constraint_objs;
        write!(f, "{} {} {}", a, self.comparison, b)
    }
}

impl<Obj: Display> Display for OfType<Obj> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} : {}", self.constraint_obj, self.of_type)
    }
}

impl<Obj: Display> Display for TypedAssign<Obj> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "{} := {} :? {}",
            self.constraint_obj, self.value, self.of_type
        )
    }
}

impl<Obj: Display> Display for PostponedCheckArgs<Obj> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} := ({}", self.constraint_obj, self.of_type)?;
        for argument in &self.arguments {
            write!(f, " {}", argument)?;
        }
        write!(f, ") ?: {}", self.the_type)
    }
}

impl Display for FindInstanceCandidate {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} : {}", self.value, self.of_type)
    }
}

impl<Obj: Display> Display for FindInstanceOF<Obj> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "Resolve instance argument {} : {}, candidates: ",
            self.constraint_obj, self.of_type
        )?;
        for argument in &self.candidates {
            write!(f, "{}, ", argument)?;
        }
        Ok(())
    }
}

impl<Obj: Display + std::fmt::Debug> Display for OutputConstraint<Obj> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use OutputConstraint::*;
        match self {
            OfType(o) => o.fmt(f),
            CmpInType {
                constraint_objs: (a, b),
                of_type,
                comparison,
            } => write!(f, "{} {} {} of type {}", a, comparison, b, of_type),
            CmpElim {
                constraint_objs: (xs, ys),
                of_type,
                polarities,
                // TODO: this can be improved, and the debug trait bound can be removed
            } => write!(f, "{:?} {:?} {:?} of type {}", xs, polarities, ys, of_type),
            JustType(j) => j.fmt(f),
            JustSort(j) => j.fmt(f),
            CmpTypes(c) => c.fmt(f),
            CmpLevels(c) => c.fmt(f),
            CmpTeles(c) => c.fmt(f),
            CmpSorts(c) => c.fmt(f),
            Guard {
                constraint,
                problem,
            } => write!(f, "{} (blocked by {})", constraint, problem),
            Assign {
                constraint_obj,
                value,
            } => write!(f, "{} := {}", constraint_obj, value),
            TypedAssign(o) => o.fmt(f),
            PostponedCheckArgs(o) => o.fmt(f),
            IsEmptyType { the_type } => write!(f, "Is empty: {}", the_type),
            SizeLtSat { the_type } => write!(f, "Not empty type of sizes: {}", the_type),
            FindInstanceOF(o) => o.fmt(f),
            PTSInstance {
                constraint_objs: (a, b),
            } => write!(f, "PTS Instance for {}, {}", a, b),
            PostponedCheckFunDef { name, of_type } => {
                write!(f, "Check definition of {} : {}", name, of_type)
            }
        }
    }
}
