use Transformer;
use transformer::{
    TransformResultHelper,
    TransformResult
};

use unicode_segmentation::UnicodeSegmentation;

pub fn capitalize(string: &str) -> String
{
    string.unicode_words()
        .map(capitalize_word).collect::<Vec<String>>()
        .join(" ")
}

fn capitalize_word(word: &str) -> String
{
    word.chars().enumerate()
        .map(|(i, c)| if i == 0 { c.to_uppercase().collect::<String>() } else { c.to_lowercase().collect() })
        .collect()
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct CapitalizeTransformer {}

impl CapitalizeTransformer
{
    pub fn new() -> CapitalizeTransformer
    {
        CapitalizeTransformer {}
    }
}

impl Transformer for CapitalizeTransformer
{
    fn transform(&self, field_value: &str, _: &str, _: usize) -> TransformResult
    {
        let result = capitalize(field_value);
        TransformResult::present(&result)
    }
}
