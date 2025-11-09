use serde::Deserialize;

use crate::base::TokenBased;
use crate::pos::InteractionPoint;

pub use self::di::*;
pub use self::give::*;
pub use self::goal::*;
pub use self::hl::*;
pub use self::oc::*;

/// Display info.
mod di;
/// About the "Give" action.
mod give;
/// Goal information.
mod goal;
/// Highlighting.
mod hl;
/// Output constraints (user goals & unsolved metas).
mod oc;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MakeCase {
    pub variant: MakeCaseVariant,
    pub interaction_point: InteractionPoint,
    pub clauses: Vec<String>,
}

/// Status information.
#[derive(Deserialize, Clone, Default, Debug, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    /// Are implicit arguments displayed?
    pub show_implicit_arguments: bool,
    /// Has the module been successfully type checked?
    pub checked: bool,
}

#[derive(Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MakeCaseVariant {
    Function,
    ExtendedLambda,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OneSolution {
    pub interaction_point: InteractionPoint,
    pub expression: String,
}

/// Agda response.
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "kind")]
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
    GiveAction(GiveAction),
    /// Response is list of printed clauses.
    MakeCase(MakeCase),
    /// Solution for one or more meta-variables.
    SolveAll {
        solutions: Vec<OneSolution>,
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
        #[serde(rename = "tokenBased")]
        token_based: TokenBased,
    },
    /// A command sent when an abort command has completed successfully.
    DoneAborting,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_status() {
        let json = r#"{
            "kind":"Status",
            "status": {
                "checked":false,
                "showImplicitArguments":false,
                "showIrrelevantArguments":false
            }
        }"#;
        let resp: Resp = serde_json::from_str(json).unwrap();
        match resp {
            Resp::Status { status } => {
                assert!(!status.show_implicit_arguments);
                assert!(!status.checked);
            }
            _ => panic!("Expected Status response"),
        }
    }

    #[test]
    fn deserialize_displayinfo() {
        let json = r#"{
            "kind":"DisplayInfo",
            "info":{
                "errors":[],
                "invisibleGoals":[
                    {
                        "kind":"JustSort",
                        "constraintObj":{
                            "name":"_0",
                            "range":[
                                {
                                    "end":{ "col":11, "line":3, "pos":50 },
                                    "start":{ "col":10, "line":3, "pos":49 }
                                }
                            ]
                        }
                    }
                ],
                "kind":"AllGoalsWarnings",
                "visibleGoals":[
                    {
                        "kind":"OfType",
                        "constraintObj":{
                            "id":0,
                            "range":[
                                {
                                    "end":{ "col":11, "line":3, "pos":50 },
                                    "start":{ "col":10, "line":3, "pos":49 }
                                }
                            ]
                        },
                        "type":"_0"
                    },
                    {
                        "kind":"OfType",
                        "constraintObj":{
                            "id":1,
                            "range":[
                                {
                                    "end":{ "col":11, "line":4, "pos":61 },
                                    "start":{ "col":10, "line":4, "pos":60 }
                                }
                            ]
                        },
                        "type":"?0"
                    }
                ],
                "warnings":[]
            }
        }"#;
        let resp: Resp = serde_json::from_str(json).unwrap();
        match resp {
            Resp::DisplayInfo { info } => {
                let info = info.unwrap();
                match info {
                    DisplayInfo::AllGoalsWarnings(agw) => {
                        assert_eq!(agw.visible_goals.len(), 2);
                        assert_eq!(agw.invisible_goals.len(), 1);
                        assert!(agw.warnings.is_empty());
                        assert!(agw.errors.is_empty());
                    }
                    _ => panic!("Expected AllGoalsWarnings"),
                }
            }
            _ => panic!("Expected Status response"),
        }
    }

    #[test]
    fn deserialize_error() {
        let json = r#"{
            "info":{
                "error":{
                    "message":"1,21-22\nGeneralizable variable SmallLib.b is not supported here\nwhen scope checking b"
                },
                "kind":"Error",
                "warnings":[]
            },
            "kind":"DisplayInfo"
        }"#;
        let resp: Resp = serde_json::from_str(json).unwrap();
        match resp {
            Resp::DisplayInfo { info } => {
                let info = info.unwrap();
                match info {
                    DisplayInfo::Error { error } => {
                        assert_eq!(
                            error.message.unwrap(),
                            "1,21-22\nGeneralizable variable SmallLib.b is not supported here\nwhen scope checking b"
                        );
                    }
                    _ => panic!("Expected AgdaError"),
                }
            }
            _ => panic!("Expected Status response"),
        }
    }
}
