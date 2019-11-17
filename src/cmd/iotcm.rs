use crate::cmd::Cmd;
use std::fmt::{Display, Error, Formatter};

impl Display for IOTCM {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "IOTCM {:?} {:?} {:?} {}",
            self.file, self.level, self.method, self.command
        )
    }
}

/// How much highlighting should be sent to the user interface?
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HighlightingLevel {
    None,
    NonInteractive,
    /// This includes both non-interactive highlighting and
    /// interactive highlighting of the expression that is currently
    /// being type-checked.
    Interactive,
}

impl Default for HighlightingLevel {
    fn default() -> Self {
        HighlightingLevel::NonInteractive
    }
}

/// How should highlighting be sent to the user interface?
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HighlightingMethod {
    /// Via stdout.
    Direct,
    /// Both via files and via stdout.
    Indirect,
}

impl Default for HighlightingMethod {
    fn default() -> Self {
        HighlightingMethod::Direct
    }
}

#[derive(Debug, Clone)]
pub struct IOTCM {
    level: HighlightingLevel,
    file: String,
    method: HighlightingMethod,
    pub command: Cmd,
}

impl IOTCM {
    pub fn new(
        level: HighlightingLevel,
        file: String,
        method: HighlightingMethod,
        command: Cmd,
    ) -> Self {
        Self {
            level,
            file,
            method,
            command,
        }
    }

    pub fn simple(file: String, command: Cmd) -> Self {
        Self::new(Default::default(), file, Default::default(), command)
    }

    /// Convert `self` into a command string.
    pub fn to_string(&self) -> String {
        format!("{}\n", self)
    }
}
