use crate::Context;
use crate::Kind;
use crate::Status;

/// Modify the [`Status`] inline for error handling.
pub trait ResultStatusExt<C, F>
where
    C: Context,
    F: Fn(C) -> C,
{
    /// Replace fields in the [`Context`] with those populated in `replacements`.
    /// ```rust
    /// # use std::path::Path;
    /// # use status::ResultStatusExt;;
    /// # type Status = status::Status;
    /// # type Result<T, E = Status> = std::result::Result<T, E>;
    /// #
    /// # fn read_file(path: &Path) -> Result<String> {
    /// #     std::fs::read_to_string(path)
    /// #         .map_err(|e| {
    /// #             Status::new("Failed to read file")
    /// #                 .with_internal(e)
    /// #         })
    /// # }
    /// #
    /// # fn main() -> Result<(), status::TerminatingStatus> {
    ///     let content = read_file(Path::new("Cargo.toml")).context_with(|c| {
    ///        c.insert("Expected value", 5)
    ///     })?;
    /// #     println!("{}", content);
    /// #     Ok(())
    /// # }
    /// ```
    fn context_with(self, replacements: F) -> Self;
}

impl<T, K, C, F> ResultStatusExt<C, F> for Result<T, Status<K, C>>
where
    K: Kind,
    C: Context,
    F: Fn(C) -> C,
{
    fn context_with(self, replacements: F) -> Self {
        self.map_err(|e| e.context_with(replacements))
    }
}
