use serde::Deserialize;
use std::fmt::{Display, Error, Formatter};
use std::ops::Range;

pub type IntPos = i32;

/// A position in the file.
#[derive(Deserialize, Clone, Copy, Default, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Pos {
    pub pos: usize,
    pub line: usize,
    pub col: usize,
}

#[derive(Deserialize, Clone, Default, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Interval {
    pub file: Option<String>,
    pub start: Pos,
    pub end: Pos,
}

impl Interval {
    pub fn range(&self) -> Range<usize> {
        self.range_shift_left(0)
    }

    pub fn range_shift_left(&self, shift: usize) -> Range<usize> {
        self.start.pos - shift..self.end.pos - shift
    }

    pub fn range_shift_right(&self, shift: usize) -> Range<usize> {
        self.start.pos + shift..self.end.pos + shift
    }
}

/// Normally, it's positive.
pub type InteractionId = i32;

/// Normally, it's also positive.
pub type ProblemId = i32;

#[derive(Deserialize, Clone, Default, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InteractionPoint {
    pub id: InteractionId,
    pub range: Vec<Interval>,
}

impl InteractionPoint {
    pub fn the_interval(&self) -> &Interval {
        debug_assert_eq!(self.range.len(), 1);
        &self.range[0]
    }
}

impl Display for InteractionPoint {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self.id)
    }
}

/// IDK why is this needed, but Emacs passes it to Agda.
/// It's fine to omit this in the commands.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AgdaRange {
    NoRange,
    Range(Interval),
}

impl Into<Option<Interval>> for AgdaRange {
    fn into(self) -> Option<Interval> {
        match self {
            AgdaRange::NoRange => None,
            AgdaRange::Range(i) => Some(i),
        }
    }
}

impl From<Option<Interval>> for AgdaRange {
    fn from(i: Option<Interval>) -> Self {
        i.map_or_else(Default::default, AgdaRange::Range)
    }
}

impl Default for AgdaRange {
    fn default() -> Self {
        AgdaRange::NoRange
    }
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct NamedMeta {
    pub name: String,
    pub range: Vec<Interval>,
}

impl Display for NamedMeta {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
