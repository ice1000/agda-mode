use crate::base::{InteractionId, Pos, Rewrite};
use std::fmt::{Display, Error, Formatter};

/// IDK why is this needed, but Emacs passes it to Agda.
/// It's fine to omit this in the commands.
#[derive(Debug, Clone)]
pub enum Range {
    NoRange,
    Range { file: String, start: Pos, end: Pos },
}

impl Default for Range {
    fn default() -> Self {
        Range::NoRange
    }
}

/// Text in the goal.
#[derive(Debug, Clone)]
pub struct GoalInput {
    id: InteractionId,
    range: Range,
    code: String,
}

impl GoalInput {
    pub fn new(id: InteractionId, range: Range, code: String) -> Self {
        GoalInput { id, range, code }
    }

    pub fn simple(id: InteractionId) -> Self {
        Self::no_range(id, String::new())
    }

    pub fn no_range(id: InteractionId, code: String) -> Self {
        Self::new(id, Default::default(), code)
    }
}

impl Display for GoalInput {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?} {} {:?}", self.id, self.range, self.code)
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "(Pn () {:?} {:?} {:?})", self.pos, self.line, self.col)
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Range::NoRange => f.write_str("noRange"),
            Range::Range { file, start, end } => write!(
                f,
                "(intervalsToRange (Just (mkAbsolute {:?})) [Interval {} {}])",
                file, start, end
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InputWithRewrite {
    pub rewrite: Rewrite,
    pub input: GoalInput,
}

impl From<GoalInput> for InputWithRewrite {
    fn from(input: GoalInput) -> Self {
        InputWithRewrite {
            input,
            rewrite: Default::default(),
        }
    }
}

impl Display for InputWithRewrite {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?} {}", self.rewrite, self.input)
    }
}
