use serde::{Deserialize, Serialize};
use std::fmt::{Display, Error, Formatter};

/// Modifier for interactive commands,
/// specifying the amount of normalization in the output.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Rewrite {
    AsIs,
    Instantiated,
    HeadNormal,
    Simplified,
    Normalised,
}

impl Default for Rewrite {
    fn default() -> Self {
        Rewrite::Simplified
    }
}

/// Modifier for the interactive computation command,
/// specifying the mode of computation and result display.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ComputeMode {
    DefaultCompute,
    IgnoreAbstract,
    UseShowInstance,
}

impl Default for ComputeMode {
    fn default() -> Self {
        ComputeMode::DefaultCompute
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Comparison {
    CmpEq,
    CmpLeq,
}

impl Display for Comparison {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(match self {
            Comparison::CmpEq => "==",
            Comparison::CmpLeq => "<=",
        })
    }
}

/// An extension of [`Comparison`](self::Comparison) to `>=`.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CompareDirection {
    DirEq,
    DirLeq,
    DirGeq,
}

impl From<Comparison> for CompareDirection {
    fn from(from: Comparison) -> Self {
        match from {
            Comparison::CmpEq => CompareDirection::DirEq,
            Comparison::CmpLeq => CompareDirection::DirLeq,
        }
    }
}

/// Polarity for equality and subtype checking.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Polarity {
    /// monotone
    Covariant,
    /// antitone
    Contravariant,
    /// no information (mixed variance)
    Invariant,
    /// constant
    Nonvariant,
}

impl Display for Polarity {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(match self {
            Polarity::Covariant => "+",
            Polarity::Contravariant => "-",
            Polarity::Invariant => "*",
            Polarity::Nonvariant => "_",
        })
    }
}

/// Modifier for interactive commands,
/// specifying whether safety checks should be ignored.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UseForce {
    /// Ignore additional checks, like termination/positivity...
    WithForce,
    /// Don't ignore any checks.
    WithoutForce,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Remove {
    Remove,
    Keep,
}

/// Is the highlighting "token-based", i.e. based only on
/// information from the lexer?
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TokenBased {
    TokenBased,
    NotOnlyTokenBased,
}

impl Default for TokenBased {
    fn default() -> Self {
        TokenBased::NotOnlyTokenBased
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HaskellBool {
    True,
    False,
}

impl From<bool> for HaskellBool {
    fn from(b: bool) -> Self {
        if b {
            HaskellBool::True
        } else {
            HaskellBool::False
        }
    }
}

impl Into<bool> for HaskellBool {
    fn into(self) -> bool {
        match self {
            HaskellBool::True => true,
            HaskellBool::False => false,
        }
    }
}

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
pub struct IPRange {
    pub start: Pos,
    pub end: Pos,
}

/// Normally, it's positive.
pub type InteractionId = i32;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, PartialEq)]
pub struct InteractionPoint {
    pub id: InteractionId,
    pub range: Vec<IPRange>,
}

impl Display for InteractionPoint {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self.id)
    }
}
