use Transformer;
use transformer::{
    TransformResultHelper,
    TransformResult
};

use regex::Regex;

lazy_static! {
    static ref ZIP_REGEX: Regex = Regex::new(r"\A(\d{5})\D*(?:(\d{4}))?\z").unwrap();
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct ZipcodeTransformer { }

impl ZipcodeTransformer {
    pub fn new() -> ZipcodeTransformer {
        ZipcodeTransformer { }
    }
}

impl Transformer for ZipcodeTransformer {
    fn transform(&self, field_value: &str, field_name: &str, record_n: usize) -> TransformResult {
        if let Some(captures) = ZIP_REGEX.captures(field_value) {
            let base_code = captures.get(1).unwrap();
            let plus_four_code = captures.get(2);
            let zipcode = if let Some(pfc) = plus_four_code {
                format!("{}-{}", base_code.as_str(), pfc.as_str())
            } else {
                base_code.as_str().to_string()
            };
            TransformResult::present(&zipcode)
        } else {
            TransformResult::error(field_value, field_name, record_n, "not a valid zipcode")
        }
    }
}
