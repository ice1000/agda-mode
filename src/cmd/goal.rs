use crate::base::Rewrite;
use crate::pos::{AgdaRange, InteractionId, Pos};
use std::fmt::{Display, Error, Formatter};

/// Text in the goal.
#[derive(Debug, Clone)]
pub struct GoalInput {
    id: InteractionId,
    range: AgdaRange,
    code: String,
}

impl GoalInput {
    pub fn new(id: InteractionId, range: AgdaRange, code: String) -> Self {
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

impl Display for AgdaRange {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            AgdaRange::NoRange => f.write_str("noRange"),
            AgdaRange::Range(r) => {
                write!(f, "(intervalsToRange ")?;
                match &r.file {
                    None => f.write_str("Nothing"),
                    Some(file) => write!(f, "(Just (mkAbsolute {:?}))", file),
                }?;
                write!(f, " [Interval {} {}])", r.start, r.end)
            }
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
