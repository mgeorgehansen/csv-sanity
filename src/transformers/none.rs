use Transformer;
use transformer::{
    TransformResultHelper,
    TransformResult
};
use newtypes::Regex;

use regex;


#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct NoneTransformer {
    regex: Regex
}

impl NoneTransformer {
    pub fn new(regex: regex::Regex) -> NoneTransformer {
        NoneTransformer { regex: Regex::from(regex) }
    }

    pub fn with_blank_matcher() -> NoneTransformer {
        Self::new(regex::Regex::new(r"\A(?:[:cntrl:]|\s)*\z").unwrap())
    }
}

impl Transformer for NoneTransformer {
    fn transform(&self, field_value: &str, _: &str, _: usize) -> TransformResult {
        if self.regex.is_match(field_value) {
            TransformResult::excluded()
        } else {
            TransformResult::present(field_value)
        }
    }
}
