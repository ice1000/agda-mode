use crate::base::{Position, TokenBased};
use serde::{Deserialize, Serialize};

/// A token highlighting information.
/// The token is somehow called `Aspect` in Agda.
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct AspectHighlight {
    pub range: (Position, Position),
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
    pub position: Position,
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
    pub info: Option<Highlighting>,
    pub filepath: Option<String>,
    pub direct: bool,
}

impl HighlightingInfo {
    /// The return value is not actually a result.
    /// I just want an `Either` type.
    pub fn into_either(self) -> Result<Highlighting, String> {
        if self.direct {
            debug_assert!(self.filepath.is_none());
            Ok(self.info.unwrap())
        } else {
            debug_assert!(self.info.is_none());
            Err(self.filepath.unwrap())
        }
    }
}
