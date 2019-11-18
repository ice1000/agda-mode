use crate::base::{IntPos, TokenBased};
use either::Either;
use serde::{Deserialize, Serialize};

/// A token highlighting information.
/// The token is somehow called `Aspect` in Agda.
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct AspectHighlight {
    pub range: (IntPos, IntPos),
    pub atoms: Vec<String>,
    pub token_based: TokenBased,
    pub note: Option<String>,
    pub definition_site: Option<DefinitionSite>,
}

/// Jump to library definition information.
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct DefinitionSite {
    pub filepath: String,
    pub position: IntPos,
}

/// A list of token highlighting information.
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct Highlighting {
    pub remove: bool,
    pub payload: Vec<AspectHighlight>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct HighlightingInfo {
    info: Option<Highlighting>,
    filepath: Option<String>,
    direct: bool,
}

impl HighlightingInfo {
    pub fn into_either(self) -> Either<Highlighting, String> {
        if self.direct {
            debug_assert!(self.filepath.is_none());
            Either::Left(self.info.unwrap())
        } else {
            debug_assert!(self.info.is_none());
            Either::Right(self.filepath.unwrap())
        }
    }
}
