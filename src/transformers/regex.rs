use Transformer;
use transformer::{
    TransformResultHelper,
    TransformResult
};
use newtypes::Regex;

use regex;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct RegexTransformer
{
    regex: Regex,
    template: String
}

impl RegexTransformer
{
    pub fn new(regex: regex::Regex, template: &str) -> RegexTransformer {
        RegexTransformer {
            regex: Regex::from(regex),
            template: template.to_string()
        }
    }
}

impl Transformer for RegexTransformer
{
    fn transform(&self, field_value: &str, field_name: &str, record_n: usize) -> TransformResult {
        if let Some(captures) = self.regex.captures(field_value) {
            let mut expansion = String::new();
            captures.expand(&self.template, &mut expansion);
            TransformResult::present(&expansion)
        } else {
            TransformResult::error(
                field_value,
                field_name,
                record_n,
                &format!("did not match pattern {}", self.regex)
            )
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct RegexMatchTransformer
{
    regex: Regex,
    negate: bool
}

impl RegexMatchTransformer
{
    pub fn matching(regex: regex::Regex) -> RegexMatchTransformer {
        RegexMatchTransformer {
            regex: Regex::from(regex),
            negate: false
        }
    }

    pub fn not_matching(regex: regex::Regex) -> RegexMatchTransformer {
        RegexMatchTransformer {
            regex: Regex::from(regex),
            negate: true
        }
    }
}

impl Transformer for RegexMatchTransformer
{
    fn transform(&self, field_value: &str, field_name: &str, record_n: usize) -> TransformResult {
        let mut is_match = self.regex.is_match(field_value);
        if self.negate {
            is_match = !is_match;
        }

        if is_match {
            TransformResult::present(field_value)
        } else {
            let reason = if self.negate {
                format!("matched exclusionary pattern {}", self.regex)
            } else {
                format!("did not match pattern {}", self.regex)
            };
            TransformResult::error(field_value, field_name, record_n, &reason)
        }
    }
}
