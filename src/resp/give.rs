use crate::base::InteractionPoint;
use either::Either;
use serde::{Deserialize, Serialize};

/// Result of a "give" action.
///
/// The structure is very mysterious.
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq)]
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

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct GiveAction {
    pub give_result: GiveResult,
    pub interaction_point: InteractionPoint,
}
