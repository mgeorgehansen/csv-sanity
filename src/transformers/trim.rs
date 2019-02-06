use Transformer;
use transformer::{
    TransformResultHelper,
    TransformResult
};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct TrimTransformer {}

impl TrimTransformer {
    pub fn new() -> TrimTransformer {
        TrimTransformer {}
    }
}

impl Transformer for TrimTransformer {
    fn transform(&self, field_value: &str, _: &str, _: usize) -> TransformResult {
        TransformResult::present(field_value.trim())
    }
}
