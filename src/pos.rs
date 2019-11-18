use serde::{Deserialize, Serialize};
use std::fmt::{Display, Error, Formatter};

pub type IntPos = i32;

/// A position in the file.
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct Pos {
    pub pos: u32,
    pub line: u32,
    pub col: u32,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct Interval {
    pub file: Option<String>,
    pub start: Pos,
    pub end: Pos,
}

/// Normally, it's positive.
pub type InteractionId = i32;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct InteractionPoint {
    pub id: InteractionId,
    pub range: Vec<Interval>,
}

impl Display for InteractionPoint {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self.id)
    }
}

/// IDK why is this needed, but Emacs passes it to Agda.
/// It's fine to omit this in the commands.
#[derive(Debug, Clone)]
pub enum Range {
    NoRange,
    Range(Interval),
}

impl Default for Range {
    fn default() -> Self {
        Range::NoRange
    }
}
