use serde::{Deserialize, Serialize};

static mut DEBUG_COMMAND: bool = false;
static mut DEBUG_RESPONSE: bool = false;

pub unsafe fn debug_command(new_value: bool) {
    DEBUG_COMMAND = new_value;
}

pub unsafe fn debug_response(new_value: bool) {
    DEBUG_RESPONSE = new_value;
}

pub unsafe fn is_debugging_command() -> bool {
    DEBUG_COMMAND
}

pub unsafe fn is_debugging_response() -> bool {
    DEBUG_RESPONSE
}

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

pub type Position = i32;
pub type InteractionPoint = u32;
