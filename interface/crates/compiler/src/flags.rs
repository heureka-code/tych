use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ExtraCompilerFlags {
    extra_includes: Vec<PathBuf>,
    macro_definitions: BTreeMap<String, Option<String>>,
}
impl Default for ExtraCompilerFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// This flag activates the calculation of valid moves after each move.
/// This is required for detecting i. e. check mate.
pub const ALWAYS_ADD_VALID_MOVES_INFORMATION: &str = "ALWAYS_ADD_VALID_MOVES_INFORMATION";

/// This flag deactivates the calculation of valid moves after the last move.
/// This is enough for detecting i. e. check mate at the end of a game.
/// This flag is stronger than [self::ALWAYS_ADD_VALID_MOVES_INFORMATION]
pub const REMOVE_VALID_MOVES_INFORMATION_FOR_LAST_STEP: &str =
    "REMOVE_VALID_MOVES_INFORMATION_FOR_LAST_STEP";

pub trait ManageCompilerFlags: From<ExtraCompilerFlags> {
    /// Transform instance into flags
    fn into_flags(self) -> ExtraCompilerFlags;

    /// Generate instance with provided path as single include path
    fn with_include(path: impl Into<PathBuf>) -> Self {
        ExtraCompilerFlags::with_include(path).into()
    }
    /// Generate a new instance without includes
    fn without_includes() -> Self {
        ExtraCompilerFlags::without_includes().into()
    }
    /// Add path as additional include directory, `-I` flag
    fn include(self, path: impl Into<PathBuf>) -> Self {
        self.into_flags().include(path).into()
    }
    /// Add include paths from iterator (multiple `-I` flags)
    fn include_multiple(self, paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> Self {
        self.into_flags().include_multiple(paths).into()
    }
    /// Define a preprocessor macro without a value, as a marker, `-D` flag
    fn define(self, macro_name: impl Into<String>) -> Self {
        self.into_flags().define(macro_name).into()
    }
    fn define_multiple(self, macro_names: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.into_flags().define_multiple(macro_names).into()
    }
    fn remove_definition(self, macro_name: impl AsRef<str>) -> Self {
        self.into_flags().remove_definition(macro_name).into()
    }
    fn remove_include(self, path: impl AsRef<Path>) -> Self {
        self.into_flags().remove_include(path).into()
    }
}

impl ExtraCompilerFlags {
    pub fn new() -> Self {
        Self::without_includes()
    }
    pub fn iter_includes(&self) -> impl Iterator<Item = &PathBuf> {
        self.extra_includes.iter()
    }
    pub fn iter_definitions(&self) -> impl Iterator<Item = (&String, &Option<String>)> {
        self.macro_definitions.iter()
    }
}

impl ManageCompilerFlags for ExtraCompilerFlags {
    /// Dummy
    fn into_flags(self) -> ExtraCompilerFlags {
        self
    }

    /// Generates new instance that has the provided path as additional include
    fn with_include(path: impl Into<PathBuf>) -> Self {
        Self::without_includes().include(path)
    }
    /// Generate a new instance without includes
    fn without_includes() -> Self {
        Self {
            extra_includes: vec![],
            macro_definitions: Default::default(),
        }
    }
    /// Add include path to instance `-I` flag
    fn include(mut self, path: impl Into<PathBuf>) -> Self {
        self.extra_includes.push(path.into());
        self
    }
    /// Add include paths from iterator (multiple `-I` flags)
    fn include_multiple(mut self, paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> Self {
        self.extra_includes
            .extend(paths.into_iter().map(|p| p.into()));
        self
    }
    /// Define a preprocessor macro without a value, as a marker, `-D` flag
    fn define(mut self, macro_name: impl Into<String>) -> Self {
        self.macro_definitions.insert(macro_name.into(), None);
        self
    }
    fn remove_definition(mut self, macro_name: impl AsRef<str>) -> Self {
        self.macro_definitions.remove(macro_name.as_ref());
        self
    }
    fn remove_include(mut self, path: impl AsRef<Path>) -> Self {
        self.extra_includes = self
            .extra_includes
            .into_iter()
            .filter(|x| x != path.as_ref())
            .collect();
        self
    }
    fn define_multiple(mut self, macro_names: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.macro_definitions
            .extend(macro_names.into_iter().map(|s| (s.into(), None)));
        self
    }
}
