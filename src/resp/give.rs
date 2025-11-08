use crate::pos::InteractionPoint;
use either::Either;
use serde::Deserialize;

/// Give action result
///
/// Comment derived from agda2-mode.el
///
/// If 'GiveResult' is 'Give_String s', then the goal is replaced by 's',
/// and otherwise the text inside the goal is retained (parenthesised
/// if 'GiveResult' is 'Give_Paren').
#[derive(Deserialize, Clone, Default, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GiveResult {
    str: Option<String>,
    paren: Option<bool>,
}

impl GiveResult {
    pub fn into_either(self) -> Either<String, bool> {
        match (self.str, self.paren) {
            (Some(s), None) => Either::Left(s),
            (None, Some(b)) => Either::Right(b),
            _ => unreachable!(),
        }
    }
}

#[derive(Deserialize, Clone, Default, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GiveAction {
    pub give_result: GiveResult,
    pub interaction_point: InteractionPoint,
}
