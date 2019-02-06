use Transformer;
use transformer::{
    TransformResultHelper,
    TransformResult
};

use time::{
    strptime
};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct DateTransformer {
    input_formats: Vec<String>,
    output_format: String
}

impl DateTransformer {
    pub fn new(input_formats: Vec<String>, output_format: &str) -> DateTransformer {
        DateTransformer {
            input_formats: input_formats,
            output_format: output_format.to_string()
        }
    }

    pub fn with_iso8601_output(input_formats: Vec<String>) -> DateTransformer {
        Self::new(input_formats, "%F")
    }
}

impl Transformer for DateTransformer {
    fn transform(&self, field_value: &str, field_name: &str, record_n: usize) -> TransformResult {
        for format in self.input_formats.iter() {
            if let Ok(time) = strptime(field_value, &format) {
                return TransformResult::present(
                    &format!("{}", time.strftime(&self.output_format).unwrap())
                );
            }
        }
        TransformResult::error(field_value, field_name, record_n, "unable to parse as date")
    }
}
