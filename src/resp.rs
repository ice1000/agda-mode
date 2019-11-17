use serde::{Deserialize, Serialize};

use crate::base::*;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct Status {
    pub show_implicit_arguments: bool,
    pub checked: bool,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct ResponseContextEntry {
    pub original_name: String,
    pub reified_name: String,
    pub binding: String,
    pub in_scope: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq)]
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

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct FindInstanceCandidate {
    #[serde(rename = "type")]
    pub of_type: String,
    pub value: String,
}

macro_rules! output_constraint {
    ($name:ident, $objTy:ty) => {
        #[serde(tag = "kind")]
        #[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
        pub enum $name {
            OfType {
                #[serde(rename = "constraintObj")]
                constraint_obj: $objTy,
                #[serde(rename = "type")]
                of_type: String,
            },
            CmpInType {
                #[serde(rename = "constraintObjs")]
                constraint_objs: ($objTy, $objTy),
                #[serde(rename = "type")]
                of_type: String,
                comparison: Comparison,
            },
            CmpElim {
                #[serde(rename = "constraintObjs")]
                constraint_objs: (Vec<$objTy>, Vec<$objTy>),
                #[serde(rename = "type")]
                of_type: String,
                polarities: Vec<Polarity>,
            },
            JustType {
                #[serde(rename = "constraintObj")]
                constraint_obj: $objTy,
            },
            JustSort {
                #[serde(rename = "constraintObj")]
                constraint_obj: $objTy,
            },
            CmpTypes {
                #[serde(rename = "constraintObjs")]
                constraint_objs: ($objTy, $objTy),
                comparison: Comparison,
            },
            CmpLevels {
                #[serde(rename = "constraintObjs")]
                constraint_objs: ($objTy, $objTy),
                comparison: Comparison,
            },
            CmpTeles {
                #[serde(rename = "constraintObjs")]
                constraint_objs: ($objTy, $objTy),
                comparison: Comparison,
            },
            CmpSorts {
                #[serde(rename = "constraintObjs")]
                constraint_objs: ($objTy, $objTy),
                comparison: Comparison,
            },
            Guard {
                constraint_objs: Box<$name>,
                problem: String,
            },
            Assign {
                #[serde(rename = "constraintObj")]
                constraint_obj: $objTy,
                value: String,
            },
            TypedAssign {
                #[serde(rename = "constraintObj")]
                constraint_obj: $objTy,
                #[serde(rename = "type")]
                of_type: String,
                value: String,
            },
            PostponedCheckArgs {
                #[serde(rename = "constraintObj")]
                constraint_obj: $objTy,
                #[serde(rename = "ofType")]
                of_type: String,
                #[serde(rename = "type")]
                the_type: String,
                arguments: Vec<String>,
            },
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
                constraint_obj: $objTy,
                #[serde(rename = "type")]
                of_type: String,
                candidates: Vec<FindInstanceCandidate>,
            },
            PTSInstance {
                #[serde(rename = "constraintObjs")]
                constraint_objs: ($objTy, $objTy),
            },
            PostponedCheckFunDef {
                name: String,
                #[serde(rename = "type")]
                of_type: String,
            },
        }
    };
}

output_constraint!(VisibleGoal, InteractionPoint);
output_constraint!(InvisibleGoal, String);

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
pub struct NormalForm {
    pub compute_mode: ComputeMode,
    pub command_state: CommandState,
    pub time: String,
    pub expr: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AllGoalsWarnings {
    pub visible_goals: Vec<VisibleGoal>,
    pub invisible_goals: Vec<InvisibleGoal>,
    pub warnings: String,
    pub errors: String,
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
    AllGoalsWarnings(AllGoalsWarnings),
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
    NormalForm(NormalForm),
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
pub struct Highlighting {
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

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct HighlightingInfo {
    pub info: Option<Highlighting>,
    pub filepath: Option<String>,
    pub direct: bool,
}

/// Agda response.
///
/// TODO: This enum is incomplete, contribution is welcomed.
#[serde(tag = "kind")]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum Resp {
    HighlightingInfo(HighlightingInfo),
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
