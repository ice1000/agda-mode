use serde::{Deserialize, Serialize};

use crate::base::InteractionPoint;
use either::Either;

/// Result of a "give" action.
///
/// The structure is very mysterious.
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct GiveResult {
    pub str: Option<String>,
    pub paren: Option<bool>,
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
