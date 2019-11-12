use crate::base::{ComputeMode, InteractionPoint, Remove, Rewrite, UseForce};

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

/// How should highlighting be sent to the user interface?
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HighlightingMethod {
    /// Via stdout.
    Direct,
    /// Both via files and via stdout.
    Indirect,
}

#[derive(Debug, Clone)]
pub struct IOTCM {
    level: HighlightingLevel,
    method: HighlightingMethod,
    command: Cmd,
}

impl IOTCM {
    pub fn new(level: HighlightingLevel, method: HighlightingMethod, command: Cmd) -> Self {
        Self {
            level,
            method,
            command,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GoalInput {
    id: InteractionPoint,
    // TODO: range
    code: String,
}

#[derive(Debug, Clone)]
pub enum Cmd {
    /// Loads the module in file `path`, using
    /// `flags` as the command-line options.
    Load {
        path: String,
        flags: Vec<String>,
    },
    /// Compiles the module in file `path` using
    /// the backend `backend`, using `flags` as the command-line options.
    Compile {
        backend: String,
        path: String,
        flags: Vec<String>,
    },
    Constraints,
    /// Show unsolved metas. If there are no unsolved metas but
    /// unsolved constraints, show those instead.
    Metas,
    /// Shows all the top-level names in the given module, along with
    /// their types. Uses the top-level scope.
    ShowModuleContentsToplevel {
        rewrite: Rewrite,
        search: String,
    },
    /// Shows all the top-level names in scope which mention all the given
    /// identifiers in their type.
    SearchAboutToplevel {
        rewrite: Rewrite,
        search: String,
    },
    /// Solve all goals whose values are determined by
    /// the constraints.
    SolveAll(Rewrite),
    /// Solve the goal at point whose value is determined by
    /// the constraints.
    SolveOne {
        rewrite: Rewrite,
        input: GoalInput,
    },
    /// Solve the goal at point by using Auto.
    AutoOne(GoalInput),
    /// Solve all goals by using Auto.
    AutoAll,
    /// Parse the given expression (as if it were defined at the
    // top-level of the current module) and infer its type.
    InferToplevel {
        /// Normalise the type?
        rewrite: Rewrite,
        code: String,
    },
    /// Parse and type check the given expression (as if it were defined
    /// at the top-level of the current module) and normalise it.
    ComputeToplevel {
        compute_mode: ComputeMode,
        code: String,
    },

    // Syntax highlighting
    //
    /// loads syntax highlighting
    /// information for the module in `path`, and asks Emacs to apply
    /// highlighting info from this file.
    ///
    /// If the module does not exist, or its module name is malformed or
    /// cannot be determined, or the module has not already been visited,
    /// or the cached info is out of date, then no highlighting information
    /// is printed.
    ///
    /// This command is used to load syntax highlighting information when a
    /// new file is opened, and it would probably be annoying if jumping to
    /// the definition of an identifier reset the proof state, so this
    /// command tries not to do that. One result of this is that the
    /// command uses the current include directories, whatever they happen
    /// to be.
    LoadHighlightingInfo {
        path: String,
    },
    /// Tells Agda to compute token-based highlighting information
    /// for the file.
    ///
    /// This command works even if the file's module name does not
    /// match its location in the file system, or if the file is not
    /// scope-correct. Furthermore no file names are put in the
    /// generated output. Thus it is fine to put source code into a
    /// temporary file before calling this command. However, the file
    /// extension should be correct.
    ///
    /// If the second argument is 'Remove', then the (presumably
    /// temporary) file is removed after it has been read.
    TokenHighlighting {
        path: String,
        remove: Remove,
    },
    /// Tells Agda to compute highlighting information for the expression just
    /// spliced into an interaction point.
    Highlight(GoalInput),
    // Implicit arguments
    //
    /// Tells Agda whether or not to show implicit arguments.
    ShowImplicitArgs(bool),
    /// Toggle display of implicit arguments.
    ToggleImplicitArgs,
    // Goal commands
    //
    /// If the range is 'noRange', then the string comes from the
    /// minibuffer rather than the goal.
    Give {
        force: UseForce,
        input: GoalInput,
    },
    Refine(GoalInput),
    Intro {
        dunno: bool,
        input: GoalInput,
    },
    RefineOrIntro {
        dunno: bool,
        input: GoalInput,
    },
    Context {
        rewrite: Rewrite,
        input: GoalInput,
    },
    HelperFunction {
        rewrite: Rewrite,
        input: GoalInput,
    },
    Infer {
        rewrite: Rewrite,
        input: GoalInput,
    },
    GoalType {
        rewrite: Rewrite,
        input: GoalInput,
    },
    /// Grabs the current goal's type and checks the expression in the hole
    /// against it. Returns the elaborated term.
    ElaborateGive {
        rewrite: Rewrite,
        input: GoalInput,
    },
    /// Displays the current goal and context.
    GoalTypeContext {
        rewrite: Rewrite,
        input: GoalInput,
    },
    /// Displays the current goal and context **and** infers the type of an
    /// expression.
    GoalTypeContextInfer {
        rewrite: Rewrite,
        input: GoalInput,
    },
    /// Grabs the current goal's type and checks the expression in the hole
    /// against it
    GoalTypeContextCheck {
        rewrite: Rewrite,
        input: GoalInput,
    },
    /// Shows all the top-level names in the given module, along with
    /// their types. Uses the scope of the given goal.
    ShowModuleContents {
        rewrite: Rewrite,
        input: GoalInput,
    },
    MakeCase(GoalInput),
    Compute {
        compute_mode: ComputeMode,
        input: GoalInput,
    },
    WhyInScope(GoalInput),
    WhyInScopeToplevel(String),
    /// Displays version of the running Agda
    ShowVersion,
    /// Abort the current computation.
    /// Does nothing if no computation is in progress.
    Abort,
}
