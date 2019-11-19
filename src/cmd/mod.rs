use std::fmt::{Display, Error as FmtError, Formatter};

use crate::base::{ComputeMode, Remove, Rewrite, UseForce};
use crate::hs::HaskellBool;

pub use self::goal::*;
pub use self::iotcm::*;

mod goal;
mod iotcm;

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
    SolveOne(InputWithRewrite),
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
    Context(InputWithRewrite),
    HelperFunction(InputWithRewrite),
    Infer(InputWithRewrite),
    GoalType(InputWithRewrite),
    /// Grabs the current goal's type and checks the expression in the hole
    /// against it. Returns the elaborated term.
    ElaborateGive(InputWithRewrite),
    /// Displays the current goal and context.
    GoalTypeContext(InputWithRewrite),
    /// Displays the current goal and context **and** infers the type of an
    /// expression.
    GoalTypeContextInfer(InputWithRewrite),
    /// Grabs the current goal's type and checks the expression in the hole
    /// against it
    GoalTypeContextCheck(InputWithRewrite),
    /// Shows all the top-level names in the given module, along with
    /// their types. Uses the scope of the given goal.
    ShowModuleContents(InputWithRewrite),
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

impl Cmd {
    pub fn load_simple(path: String) -> Self {
        Cmd::Load {
            path,
            flags: vec![],
        }
    }

    /// Produces [CurrentGoal](crate::resp::GoalInfo::CurrentGoal).
    pub fn goal_type(input: GoalInput) -> Self {
        Cmd::GoalType(From::from(input))
    }

    pub fn search_module(search: String) -> Self {
        Cmd::ShowModuleContentsToplevel {
            rewrite: Default::default(),
            search,
        }
    }

    /// Produces [InferredType](crate::resp::GoalInfo::InferredType).
    pub fn infer(input: GoalInput) -> Self {
        Cmd::Infer(From::from(input))
    }

    pub fn give(input: GoalInput) -> Self {
        Cmd::Give {
            force: UseForce::WithoutForce,
            input,
        }
    }
}

impl Display for Cmd {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        use Cmd::*;
        match self {
            Load { path, flags } => write!(f, "( Cmd_load {:?} {:?} )", path, flags),
            Compile {
                backend,
                path,
                flags,
            } => write!(f, "( Cmd_compile {:?} {:?} {:?} )", backend, path, flags),
            Constraints => f.write_str("Cmd_constraints"),
            Metas => f.write_str("Cmd_metas"),
            ShowModuleContentsToplevel { rewrite, search } => write!(
                f,
                "( Cmd_show_module_contents_toplevel {:?} {:?} )",
                rewrite, search
            ),
            SearchAboutToplevel { rewrite, search } => write!(
                f,
                "( Cmd_search_about_toplevel {:?} {:?} )",
                rewrite, search
            ),
            SolveAll(rewrite) => write!(f, "( Cmd_solveAll {:?} )", rewrite),
            SolveOne(info) => write!(f, "( Cmd_solveOne {} )", info),
            AutoOne(input) => write!(f, "( Cmd_autoOne {} )", input),
            AutoAll => f.write_str("Cmd_autoAll"),
            InferToplevel { rewrite, code } => {
                write!(f, "( Cmd_infer_toplevel {:?} {:?} )", rewrite, code)
            }
            ComputeToplevel { compute_mode, code } => {
                write!(f, "( Cmd_compute_toplevel {:?} {:?} )", compute_mode, code)
            }
            LoadHighlightingInfo { path } => write!(f, "( Cmd_load_highlighting_info {:?} )", path),
            TokenHighlighting { path, remove } => {
                write!(f, "( Cmd_tokenHighlighting {:?} {:?} ", path, remove)
            }
            Highlight(input) => write!(f, "( Cmd_highlight {} )", input),
            ShowImplicitArgs(show) => {
                write!(f, "( ShowImplicitArgs {:?} )", HaskellBool::from(*show))
            }
            ToggleImplicitArgs => f.write_str("ToggleImplicitArgs"),
            Give { force, input } => write!(f, "( Cmd_give {:?} {} )", force, input),
            Refine(input) => write!(f, "( Cmd_refine {} )", input),
            Intro { dunno, input } => {
                write!(f, "( Cmd_intro {:?} {} )", HaskellBool::from(*dunno), input)
            }
            RefineOrIntro { dunno, input } => write!(
                f,
                "( Cmd_refine_or_intro {:?} {} )",
                HaskellBool::from(*dunno),
                input
            ),
            Context(info) => write!(f, "( Cmd_context {} )", info),
            HelperFunction(info) => write!(f, "( Cmd_helper_function {} )", info),
            Infer(info) => write!(f, "( Cmd_infer {} )", info),
            GoalType(info) => write!(f, "( Cmd_goal_type {} )", info),
            ElaborateGive(info) => write!(f, "( Cmd_elaborate_give {} )", info),
            GoalTypeContext(info) => write!(f, "( Cmd_goal_type_context {} )", info),
            GoalTypeContextInfer(info) => write!(f, "( Cmd_goal_type_context_infer {} )", info),
            GoalTypeContextCheck(info) => write!(f, "( Cmd_goal_type_context_check {} )", info),
            ShowModuleContents(info) => write!(f, "( Cmd_show_module_contents {} )", info),
            MakeCase(input) => write!(f, "( Cmd_make_case {} )", input),
            Compute {
                compute_mode,
                input,
            } => write!(f, "( Cmd_compute {:?} {} )", compute_mode, input),
            WhyInScope(input) => write!(f, "( Cmd_why_in_scope {} )", input),
            WhyInScopeToplevel(name) => write!(f, "( Cmd_why_in_scope_toplevel {:?} )", name),
            ShowVersion => f.write_str("Cmd_show_version"),
            Abort => f.write_str("Cmd_abort"),
        }
    }
}
