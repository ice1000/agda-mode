/// Modifier for interactive commands,
/// specifying the amount of normalization in the output.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Rewrite {
    AsIs,
    Instantiated,
    HeadNormal,
    Simplified,
    Normalised,
}

/// Modifier for the interactive computation command,
/// specifying the mode of computation and result display.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ComputeMode {
    DefaultCompute,
    IgnoreAbstract,
    UseShowInstance,
}

pub type InteractionPoint = u32;
