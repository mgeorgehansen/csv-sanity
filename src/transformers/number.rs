use Transformer;
use transformer::{
    TransformResultHelper,
    TransformResult
};

use regex::Regex;

lazy_static! {
    static ref INTEGER_REGEX: Regex = Regex::new(r"\A(:?0|[1-9]\d*)\z").unwrap();
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct NumberTransformer { }

impl NumberTransformer {
    pub fn match_integer() -> NumberTransformer {
        NumberTransformer { }
    }
}

impl Transformer for NumberTransformer {
    fn transform(&self, field_value: &str, field_name: &str, record_n: usize) -> TransformResult {
        if INTEGER_REGEX.is_match(field_value) {
            TransformResult::present(field_value)
        } else {
            TransformResult::error(field_value, field_name, record_n, "not a valid number")
        }
    }
}
