use transformer::{
    Transformer,
    TransformResult,
};

mod trim;
pub use self::trim::TrimTransformer;

mod none;
pub use self::none::NoneTransformer;

mod regex;
pub use self::regex::{
    RegexTransformer,
    RegexMatchTransformer
};

mod capitalize;
pub use self::capitalize::{
    CapitalizeTransformer,
    capitalize
};

mod email;
pub use self::email::EmailTransformer;

mod number;
pub use self::number::NumberTransformer;

mod date;
pub use self::date::DateTransformer;

mod choice;
pub use self::choice::ChoiceTransformer;

mod zipcode;
pub use self::zipcode::ZipcodeTransformer;

mod phone_number;
pub use self::phone_number::PhoneNumberTransformer;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub enum Transformers
{
    Trim(TrimTransformer),
    None(NoneTransformer),
    Regex(RegexTransformer),
    RegexMatch(RegexMatchTransformer),
    Capitalize(CapitalizeTransformer),
    Email(EmailTransformer),
    Number(NumberTransformer),
    Date(DateTransformer),
    Choice(ChoiceTransformer),
    Zipcode(ZipcodeTransformer),
    PhoneNumber(PhoneNumberTransformer),
}

impl Transformer for Transformers {
    fn transform(&self, field_value: &str, field_name: &str, record_n: usize) -> TransformResult {
        use self::Transformers::*;

        match *self {
            Trim(ref t) => t.transform(field_value, field_name, record_n),
            None(ref t) => t.transform(field_value, field_name, record_n),
            Regex(ref t) => t.transform(field_value, field_name, record_n),
            RegexMatch(ref t) => t.transform(field_value, field_name, record_n),
            Capitalize(ref t) => t.transform(field_value, field_name, record_n),
            Email(ref t) => t.transform(field_value, field_name, record_n),
            Number(ref t) => t.transform(field_value, field_name, record_n),
            Date(ref t) => t.transform(field_value, field_name, record_n),
            Choice(ref t) => t.transform(field_value, field_name, record_n),
            Zipcode(ref t) => t.transform(field_value, field_name, record_n),
            PhoneNumber(ref t) => t.transform(field_value, field_name, record_n)
        }
    }
}
