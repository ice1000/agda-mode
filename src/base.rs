use serde::Deserialize;
use std::fmt::{Display, Error, Formatter};

/// Modifier for interactive commands,
/// specifying the amount of normalization in the output.
#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UseForce {
    /// Ignore additional checks, like termination/positivity...
    WithForce,
    /// Don't ignore any checks.
    WithoutForce,
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Remove {
    Remove,
    Keep,
}

/// Is the highlighting "token-based", i.e. based only on
/// information from the lexer?
#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TokenBased {
    TokenBased,
    NotOnlyTokenBased,
}

impl Default for TokenBased {
    fn default() -> Self {
        TokenBased::NotOnlyTokenBased
    }
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Hiding {
    YesOverlap,
    NoOverlap,
    Hidden,
    NotHidden,
}

/// A function argument can be relevant or irrelevant.
/// See "Agda.TypeChecking.Irrelevance".
#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Relevance {
    /// The argument is (possibly) relevant at compile-time.
    Relevant,
    /// The argument may never flow into evaluation position.
    /// Therefore, it is irrelevant at run-time.
    /// It is treated relevantly during equality checking.
    NonStrict,
    /// The argument is irrelevant at compile- and runtime.
    Irrelevant,
}

/// Cohesion modalities
/// see "Brouwer's fixed-point theorem in real-cohesive homotopy type theory" (arXiv:1509.07584)
/// types are now given an additional topological layer which the modalities interact with.
#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Cohesion {
    /// Same points, discrete topology, idempotent comonad, box - like.
    Flat,
    /// Identity modality.
    Continuous,
    /// Same points, codiscrete topology, idempotent monad, diamond-like.
    Sharp,
    /// Single point space, artificially added for Flat left-composition.
    Squash,
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
