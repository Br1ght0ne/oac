use thiserror::Error;

/// A convenience wrapper for [std::result::Result] where E is [Error].
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Description of all domain-specific errors.
#[derive(Debug, Error)]
pub enum Error {
    /// `reference` is not pointing to a valid location in the document.
    #[error("missing reference target for reference {reference}")]
    MissingReferenceTarget {
        #[allow(missing_docs)]
        reference: String,
    },
}
