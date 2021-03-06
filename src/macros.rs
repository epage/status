/// Return early with an error.
///
/// This macro is equivalent to `return Err(From::from($err))`.
///
/// # Example
///
/// ```
/// # fn has_permission(user: usize, resource: usize) -> bool {
/// #     true
/// # }
/// #
/// # fn writer() -> Result<(), status::Status> {
/// #     let user = 0;
/// #     let resource = 0;
/// #
/// if !has_permission(user, resource) {
///     status::bail!("permission denied for accessing resource");
/// }
/// #     Ok(())
/// # }
/// ```
///
/// ```
/// # #[derive(Copy, Clone, Debug, derive_more::Display)]
/// # enum ErrorKind {
/// #   #[display(fmt = "Failed to read file")]
/// #   Read,
/// #   #[display(fmt = "Failed to parse")]
/// #   Parse,
/// # }
/// # type Status = status::Status<ErrorKind>;
/// # type Result<T, E = Status> = std::result::Result<T, E>;
///
/// fn read_file() -> Result<()> {
///     status::bail!(ErrorKind::Read);
/// }
/// ```
#[macro_export]
macro_rules! bail {
    ($msg:literal $(,)?) => {
        return ::core::result::Result::Err($crate::Status::new($msg));
    };
    ($err:expr $(,)?) => {
        return ::core::result::Result::Err($crate::Status::new($err));
    };
}

/// Return early with an error if a condition is not satisfied.
///
/// This macro is equivalent to `if !$cond { return Err(From::from($err)); }`.
///
/// Analogously to `assert!`, `ensure!` takes a condition and exits the function
/// if the condition fails. Unlike `assert!`, `ensure!` returns an `Error`
/// rather than panicking.
///
/// # Example
///
/// ```
/// # fn writer() -> Result<(), status::Status> {
/// #     let user = 0;
/// #
/// status::ensure!(user == 0, "only user 0 is allowed");
/// #     Ok(())
/// # }
/// ```
///
/// ```
/// # #[derive(Copy, Clone, Debug, derive_more::Display)]
/// # enum ErrorKind {
/// #   #[display(fmt = "Failed to read file")]
/// #   Read,
/// #   #[display(fmt = "Failed to parse")]
/// #   Parse,
/// # }
/// # type Status = status::Status<ErrorKind>;
/// # type Result<T, E = Status> = std::result::Result<T, E>;
/// #
/// # const MAX_DEPTH: usize = 1;
/// #
/// fn read_file() -> Result<()> {
/// #     let depth = 0;
/// #
///     status::ensure!(depth <= MAX_DEPTH, ErrorKind::Read);
/// #     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $msg:literal $(,)?) => {
        if !$cond {
            return ::core::result::Result::Err($crate::Status::new($msg));
        }
    };
    ($cond:expr, $err:expr $(,)?) => {
        if !$cond {
            return ::core::result::Result::Err($crate::Status::new($err));
        }
    };
}
