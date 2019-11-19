use serde::Deserialize;

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
