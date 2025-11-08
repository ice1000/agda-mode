use crate::base::TokenBased;
use crate::pos::IntPos;
use either::Either;
use serde::Deserialize;

/// A token highlighting information.
/// The token is somehow called `Aspect` in Agda.
#[derive(Deserialize, Clone, Default, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AspectHighlight {
    pub range: (IntPos, IntPos),
    pub atoms: Vec<String>,
    pub token_based: TokenBased,
    pub note: Option<String>,
    pub definition_site: Option<DefinitionSite>,
}

/// Jump to library definition information.
#[derive(Deserialize, Clone, Default, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DefinitionSite {
    pub filepath: String,
    pub position: IntPos,
}

/// A list of token highlighting information.
#[derive(Deserialize, Clone, Default, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Highlighting {
    pub remove: bool,
    pub payload: Vec<AspectHighlight>,
}

#[derive(Deserialize, Clone, Default, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
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
