use Transformer;
use transformer::{
    TransformResultHelper,
    TransformResult
};

use regex::Regex;

lazy_static! {
    static ref NANP_REGEX: Regex = Regex::new(r"\A(?:\+?1)?\D*\(?(?P<area>\d{3})\)?\D*(?P<exchange>\d{3})\D*(?P<subscriber>\d{4})\z").unwrap();
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct PhoneNumberTransformer { }

impl PhoneNumberTransformer {
    pub fn expect_nanp_format() -> PhoneNumberTransformer {
        PhoneNumberTransformer { }
    }
}

impl Transformer for PhoneNumberTransformer {
    fn transform(&self, field_value: &str, field_name: &str, record_n: usize) -> TransformResult {
        if let Some(captures) = NANP_REGEX.captures(field_value) {
            let area_code = captures.name("area").unwrap().as_str();
            let exchange_code = captures.name("exchange").unwrap().as_str();
            let subscriber_number = captures.name("subscriber").unwrap().as_str();
            let phone_number = format!("+1 {} {} {}", area_code, exchange_code, subscriber_number);
            TransformResult::present(&phone_number)
        } else {
            TransformResult::error(field_value, field_name, record_n, "not a valid NANP format phone number")
        }
    }
}
