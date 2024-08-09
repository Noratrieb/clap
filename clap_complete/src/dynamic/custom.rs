use std::any::type_name;
use std::sync::Arc;

use clap::builder::ArgExt;

use super::CompletionCandidate;

/// A wrapper for custom completer
///
/// # Example
///
/// ```rust
/// use clap::Parser;
/// use clap_complete::dynamic::{ArgValueCompleter, CompletionCandidate};
///
/// #[derive(Debug, Parser)]
/// struct Cli {
///     #[arg(long, add = ArgValueCompleter::new(|| { vec![
///         CompletionCandidate::new("foo"),
///         CompletionCandidate::new("bar"),
///         CompletionCandidate::new("baz")] }))]
///     custom: Option<String>,
/// }
/// ```
#[derive(Clone)]
pub struct ArgValueCompleter(Arc<dyn CustomCompleter>);

impl ArgValueCompleter {
    /// Create a new `ArgValueCompleter` with a custom completer
    pub fn new<C>(completer: C) -> Self
    where
        C: CustomCompleter + 'static,
    {
        Self(Arc::new(completer))
    }

    /// See [`CompletionCandidate`] for more information.
    pub fn completions(&self) -> Vec<CompletionCandidate> {
        self.0.completions()
    }
}

impl std::fmt::Debug for ArgValueCompleter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(type_name::<Self>())
    }
}

impl ArgExt for ArgValueCompleter {}

/// User-provided completion candidates for an argument.
///
/// This is useful when predefined value hints are not enough.
pub trait CustomCompleter: Send + Sync {
    /// All potential candidates for an argument.
    ///
    /// See [`CompletionCandidate`] for more information.
    fn completions(&self) -> Vec<CompletionCandidate>;
}

impl<F> CustomCompleter for F
where
    F: Fn() -> Vec<CompletionCandidate> + Send + Sync,
{
    fn completions(&self) -> Vec<CompletionCandidate> {
        self()
    }
}