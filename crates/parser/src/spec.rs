use crate::Reference;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Version of the OAC spec this crate implements.
pub const VERSION: &str = "0.2.1";

/// OAC document is a JSON document described in RFC8259.
#[allow(missing_docs)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Document<'a> {
    pub openautocompletion: OpenAutoCompletion,
    pub components: Components,
    pub cli: Cli<'a>,
}

/// Metadata about the spec used.
#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAutoCompletion {
    /// `major.minor` portion of OAC specification, e.g. `1.0`.
    pub version: String,
}

impl Default for OpenAutoCompletion {
    fn default() -> Self {
        Self {
            version: String::from(VERSION),
        }
    }
}

/// Root for all reusable components for use across OAC document.
/// These components are allowed to be referenced.
#[allow(missing_docs)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Components {
    pub arguments: std::option::Option<HashMap<String, Argument>>,
    pub options: std::option::Option<HashMap<String, Option>>,
    pub commands: std::option::Option<HashMap<String, Command>>,
}

/// Specification of a positional argument.
/// Examples: (`docker pull`) `IMAGE`, etc.
#[derive(Debug, Deserialize, Serialize)]
pub struct Argument {
    /// Name that identifies this argument.
    pub name: String,
    /// Description of this argument.
    pub description: std::option::Option<String>,
}

/// Specification of an option - a boolean flag.
/// Examples: `--verbose`, `--dry-run`, etc.
#[derive(Debug, Deserialize, Serialize)]
pub struct Option {
    /// Short option names as they appear in CLI, without prefix.
    /// Example: ["s"]
    pub names_short: std::option::Option<Vec<String>>,
    /// Long option names as they appear in CLI, without prefix.
    /// This array SHOULD contain:
    /// - POSIX long options (long, double-dashed).
    /// - Legacy (long, single-dashed)
    /// - Windows-style (long and short, starting with forward slash "/")
    /// - PowerShell-style options (long, single-dashed)
    /// Example: ["simulate", "just-print", "dry-run", "recon", "no-act"]
    pub names_long: std::option::Option<Vec<String>>,
    /// Description of this item.
    pub description: std::option::Option<String>,
}

/// Specification of a subcommand.
/// Examples: (`http`) `get`, `post`, etc.
#[derive(Debug, Deserialize, Serialize)]
pub struct Command {
    /// Various names, that are used as command-line subcommands.
    /// This array MUST contain at least.
    /// The first element SHOULD be the most unambigous command.
    /// For example, when ci and clean-install have the same meaning,
    /// then the resulting array will be: ["clean-install", "ci"]
    pub names: Vec<String>,
    /// Description of this item.
    pub description: std::option::Option<String>,
}

/// A `Pattern` is a "place" in the completion DAG
/// where a component can appear.
#[allow(missing_docs)]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Pattern<'a> {
    Command {
        command: Reference<'a, Command>,
        #[serde(flatten)]
        common: PatternCommon,
    },
    Argument {
        argument: Reference<'a, Argument>,
        #[serde(flatten)]
        common: PatternCommon,
    },
    Option {
        option: Reference<'a, Option>,
        /// Pattern that could be used to parse the argument for the option.
        argument: std::option::Option<Box<Pattern<'a>>>,
        #[serde(flatten)]
        common: PatternCommon,
    },
    /// A recursive structure, representing a parsing DAG.
    Group {
        /// If `true`, then all the group members are mutually exclusive.
        ///
        /// If `false`, then all the group members are required
        /// (except for members explicitly marked as optional).
        #[serde(default)]
        exclusive: bool,
        /// Patterns that this group contains.
        patterns: Vec<Pattern<'a>>,
        #[serde(flatten)]
        common: PatternCommon,
    },
}

/// Common fields shared by all pattern types.
#[derive(Debug, Deserialize, Serialize)]
pub struct PatternCommon {
    /// Marks the pattern that it could be repeated multiple times
    #[serde(default)]
    pub repeated: bool,
    /// Marks the pattern that it could be omitted from command
    #[serde(default)]
    pub optional: bool,
}

/// Description of the general properties of the program.
#[derive(Debug, Deserialize, Serialize)]
pub struct Cli<'a> {
    /// The command itself, as it appears in the shell.
    pub name: String,
    /// Separators that should appear after long options.
    #[serde(default = "default_option_separators_long")]
    pub option_separators_long: Vec<String>,
    /// Separators that should appear after short options.
    #[serde(default = "default_option_separators_short")]
    pub option_separators_short: Vec<String>,
    /// Prefix for long options.
    #[serde(default = "default_option_prefix_long")]
    pub option_prefix_long: String,
    /// Prefix for short options.
    #[serde(default = "default_option_prefix_short")]
    pub option_prefix_short: String,
    /// All usage patterns for given command.
    pub pattern_groups: Vec<Pattern<'a>>,
}

fn default_option_separators_long() -> Vec<String> {
    vec!["=".into(), " ".into()]
}

fn default_option_separators_short() -> Vec<String> {
    vec![" ".into(), "".into()]
}

fn default_option_prefix_long() -> String {
    String::from("--")
}

fn default_option_prefix_short() -> String {
    String::from("-")
}
