use serde::Deserialize;
use std::fmt::{Display, Error, Formatter};

pub type IntPos = i32;

/// A position in the file.
#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Clone, Copy, Default, Debug, Eq, PartialEq)]
pub struct Pos {
    pub pos: usize,
    pub line: usize,
    pub col: usize,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct Interval {
    pub file: Option<String>,
    pub start: Pos,
    pub end: Pos,
}

/// Normally, it's positive.
pub type InteractionId = i32;

/// Normally, it's also positive.
pub type ProblemId = i32;

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Clone, Default, Debug, Eq, PartialEq)]
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
pub enum AgdaRange {
    NoRange,
    Range(Interval),
}

impl Default for AgdaRange {
    fn default() -> Self {
        AgdaRange::NoRange
    }
}
