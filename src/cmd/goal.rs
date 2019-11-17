use crate::base::{InteractionPoint, Rewrite};
use std::fmt::{Display, Error, Formatter};

/// A position in the file.
#[derive(Debug, Clone)]
pub struct Pn {
    pub offset: u32,
    pub line: u32,
    pub column: u32,
}

/// IDK why is this needed, but Emacs passes it to Agda.
/// It's fine to omit this in the commands.
#[derive(Debug, Clone)]
pub enum Range {
    NoRange,
    Range { file: String, start: Pn, end: Pn },
}

impl Default for Range {
    fn default() -> Self {
        Range::NoRange
    }
}

/// Text in the goal.
#[derive(Debug, Clone)]
pub struct GoalInput {
    id: InteractionPoint,
    range: Range,
    code: String,
}

impl GoalInput {
    pub fn new(id: InteractionPoint, range: Range, code: String) -> Self {
        GoalInput { id, range, code }
    }

    pub fn simple(id: InteractionPoint) -> Self {
        Self::no_range(id, String::new())
    }

    pub fn no_range(id: InteractionPoint, code: String) -> Self {
        Self::new(id, Default::default(), code)
    }
}

impl Display for GoalInput {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?} {} {:?}", self.id, self.range, self.code)
    }
}

impl Display for Pn {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "(Pn () {:?} {:?} {:?})",
            self.offset, self.line, self.column
        )
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

impl Display for InputWithRewrite {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?} {}", self.rewrite, self.input)
    }
}
