use Transformer;
use transformer::{
    TransformResultHelper,
    TransformResult
};

use regex::Regex;

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(r"(?i)\A[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\z").unwrap();
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct EmailTransformer {}

impl EmailTransformer {
    pub fn new() -> EmailTransformer {
        EmailTransformer {}
    }
}

impl Transformer for EmailTransformer {
    fn transform(&self, field_value: &str, field_name: &str, record_n: usize) -> TransformResult {
        if EMAIL_REGEX.is_match(field_value) {
            TransformResult::present(&field_value.to_lowercase())
        } else {
            TransformResult::error(field_value, field_name, record_n, "invalid email address")
        }
    }
}
