use Transformer;
use transformer::{
    TransformResultHelper,
    TransformResult
};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct ChoiceTransformer {
    choices: Vec<String>,
}

impl ChoiceTransformer
{
    pub fn new(choices: Vec<String>) -> ChoiceTransformer
    {
        ChoiceTransformer {
            choices: choices,
        }
    }
}

impl Transformer for ChoiceTransformer
{
    fn transform(&self, field_value: &str, field_name: &str, record_n: usize) -> TransformResult
    {
        if self.choices.contains(&field_value.to_string()) {
            TransformResult::present(&field_value)
        } else {
            TransformResult::error(
                field_value,
                field_name,
                record_n,
                &format!("not in valid choices {:?}", self.choices)
            )
        }
    }
}
