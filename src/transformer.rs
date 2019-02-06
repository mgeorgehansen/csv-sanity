//! Traits and types that define transformations on CSV record fields.

use std::result;
use std::error;
use std::fmt::{
    self,
    Formatter,
    Display,
};

/// `Result` for the transformation of a CSV record's field, either an `Option<String>` if
/// successfully transformed or a `TransformError` if unsuccessful.
pub type TransformResult = result::Result<Option<String>, TransformError>;

/// Helper trait with a few useful utility methods for constructing `TransformResult`.
pub trait TransformResultHelper
{
    /// Construct a `TransformResult` that represents a successful transformation of a CSV record's
    /// field with a non-empty value.
    fn present(value: &str) -> TransformResult {
        Ok(Some(value.to_string()))
    }

    /// Construct a `TransformResult` that represents a successful tranformation of a CSV record's
    /// field with an empty value.
    fn excluded() -> TransformResult {
        Ok(None)
    }

    /// Construct a `TransformResult` that represents a failed transformation of a CSV record's
    /// field with a descritive error reason.
    ///
    /// An error reason should be a short, single sentence without punctuation or capitization,
    /// e.g. "not a valid email address" instead of "The email address was invalid.".
    ///
    /// ```
    /// use csv_sanity::transformer::{
    ///     TransformResult,
    ///     TransformError,
    ///     TransformResultHelper,
    /// };
    ///
    /// let result = TransformResult::error("jak,.@hot mail.com", "Email", 0, "not a valid email address");
    /// assert_eq!(result, Err(TransformError {
    ///     field_value: "jak,.@hot mail.com".to_string(),
    ///     field_name: "Email".to_string(),
    ///     record_n: 0,
    ///     reason: "not a valid email address".to_string(),
    /// }));
    /// ```
    fn error(field_value: &str, field_name: &str, record_n: usize, reason: &str) -> TransformResult {
        Err(
            TransformError {
                field_value: field_value.to_string(),
                field_name: field_name.to_string(),
                record_n: record_n,
                reason: reason.to_string(),
            }
        )
    }
}

impl TransformResultHelper for TransformResult {}

pub trait Transformer
{
    fn transform(&self, field_value: &str, field_name: &str, record_n: usize) -> TransformResult;
}

#[derive(RustcEncodable, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct TransformError
{
    pub record_n: usize,
    pub field_name: String,
    pub field_value: String,
    pub reason: String,
}

impl Display for TransformError
{
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "failed to transform field: {}", self.reason)
    }
}

impl error::Error for TransformError
{
    fn description(&self) -> &str {
        &self.reason
    }
}
