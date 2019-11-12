use serde::{Deserialize, Serialize};

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

/// Modifier for the interactive computation command,
/// specifying the mode of computation and result display.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ComputeMode {
    DefaultCompute,
    IgnoreAbstract,
    UseShowInstance,
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

pub type InteractionPoint = u32;
