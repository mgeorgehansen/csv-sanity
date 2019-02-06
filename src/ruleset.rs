use Transformer;
use transformer::{
    TransformResult,
    TransformError,
};
use transformers::{
    Transformers,
    TrimTransformer,
    NoneTransformer,
};

use std::hash::{
    Hash,
    Hasher,
};
use std::iter::FromIterator;
use std::cmp::Ordering;
use std::collections::{
    BinaryHeap,
    HashSet,
};
use std::error;
use std::fmt::{
    self,
    Formatter,
    Display,
};

/// Applicability of a `Rule` determining which CSV record's fields it can be applied to.
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub enum Applicability {
    /// Applicable to all CSV record fields.
    Global,
    /// Applicable to a subset of a CSV record's fields, specified by field name.
    Fields {
        field_names: HashSet<String>
    }
}

impl Hash for Applicability {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher {
        use self::Applicability::*;
        match *self {
            Global => (self as *const Applicability).hash(state), // FIXME: Is this the correct way to hash an empty enum variant?
            Fields { ref field_names } => field_names.iter().collect::<Vec<&String>>().hash(state)
        }
    }
}

fn priority_is_default(priority: &isize) -> bool {
    priority == &0
}

/// A `Transformer` paired with `Applicability` and a priority which can be applied to fields in a
/// CSV record.
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct Rule
{
    applicability: Applicability,
    transformer: Transformers,
    #[serde(default, skip_serializing_if="priority_is_default")]
    priority: isize
}

impl Rule
{
    /// Construct a new `Rule` whoe `Transformer` is applicable to one or more CSV record's fields
    /// referenced by name with the default priority of 0.
    ///
    /// # Examples
    /// ```
    /// use csv_sanity::Rule;
    /// use csv_sanity::transformers::*;
    ///
    /// let rule = Rule::for_fields(&["First Name", "Last Name"], Transformers::Capitalize(
    ///     CapitalizeTransformer::new()
    /// ));
    /// ```
    pub fn for_fields(field_names: &[&str], transformer: Transformers) -> Rule {
        Self::for_fields_with_priority(field_names, transformer, Default::default())
    }

    /// Construct a new `Rule` whoe `Transformer` is applicable to one or more CSV record's fields
    /// referenced by name with the specified priority.
    ///
    /// # Examples
    /// ```
    /// use csv_sanity::Rule;
    /// use csv_sanity::transformers::*;
    ///
    /// let rule = Rule::for_fields_with_priority(&["Fist Name", "Last Name"], Transformers::Capitalize(
    ///     CapitalizeTransformer::new()
    /// ), 10);
    /// ```
    pub fn for_fields_with_priority(field_names: &[&str], transformer: Transformers, priority: isize) -> Rule {
        Rule {
            applicability: Applicability::Fields { field_names: field_names.iter().map(|s| s.to_string()).collect() },
            transformer: transformer,
            priority: priority
        }
    }

    /// Construct a new `Rule` applicable to all of a CSV record's fields with the default priority
    /// of 0.
    ///
    /// # Examples
    /// ```
    /// use csv_sanity::Rule;
    /// use csv_sanity::transformers::*;
    ///
    /// let rule = Rule::global(Transformers::Capitalize(
    ///     CapitalizeTransformer::new()
    /// ));
    /// ```
    pub fn global(transformer: Transformers) -> Rule {
        Self::global_with_priority(transformer, Default::default())
    }

    /// Construct a new `Rule` applicable to all of a CSV record's fields with the specified
    /// priority.
    ///
    /// # Examples
    /// ```
    /// use csv_sanity::Rule;
    /// use csv_sanity::transformers::*;
    ///
    /// let rule = Rule::global_with_priority(Transformers::Capitalize(
    ///     CapitalizeTransformer::new()
    /// ), 10);
    /// ```
    pub fn global_with_priority(transformer: Transformers, priority: isize) -> Rule {
        Rule {
            applicability: Applicability::Global,
            transformer: transformer,
            priority: priority
        }
    }

    /// Apply this rule to a CSV record's field, returning the resulting `TransformResult`.
    ///
    /// # Examples
    /// ```
    /// use csv_sanity::Rule;
    /// use csv_sanity::transformers::*;
    ///
    /// let field = "JOHN";
    /// let field_name = "First Name";
    ///
    /// let rule = Rule::for_fields(&["First Name", "Last Name"], Transformers::Capitalize(
    ///     CapitalizeTransformer::new()
    /// ));
    /// rule.apply(field, field_name, 1);
    /// ```
    pub fn apply(&self, field_value: &str, field_name: &str, record_n: usize) -> TransformResult {
        // XXX: Does the applicability check belong inside the apply method? Or should the caller
        //   decide?
        match self.applicability {
            Applicability::Global => self.transformer.transform(field_value, field_name, record_n),
            Applicability::Fields { ref field_names } if field_names.contains(&field_name.to_string()) => {
                self.transformer.transform(field_value, field_name, record_n)
            },
            _ => Ok(Some(field_value.to_string()))
        }
    }
}

impl Ord for Rule
{
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for Rule
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// An ordered set of `Rule`s sorted by priority.
///
/// # Examples
/// ```
/// use csv_sanity::{
///     Ruleset,
///     Rule,
///     TransformedRecord,
/// };
/// use csv_sanity::transformers::*;
/// let ruleset = {
///     let mut r = Ruleset::new();
///     r.add_rule(Rule::for_fields(&["First Name", "Last Name"], Transformers::Capitalize(
///         CapitalizeTransformer::new()
///     )));
///     r.add_rule(Rule::for_fields(&["Email"], Transformers::Email(
///         EmailTransformer::new()
///     )));
///     r
/// };
/// let headers = vec!["Id", "First Name", "Last Name", "Email"].iter().map(|s| s.to_string()).collect();
/// let record = vec!["1", " JOHN", "SNOW  ", "\t   JSNOW@EXAMPLE.COM "].iter().map(|s| s.to_string()).collect();
/// let transformed_record = ruleset.apply_rules(&headers, &record, 1);
/// assert_eq!(TransformedRecord {
///     field_values: vec!["1", "John", "Snow", "jsnow@example.com"].iter().map(|s| Some(s.to_string())).collect(),
///     errors: Vec::new(),
/// }, transformed_record);
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ruleset {
    rules: BinaryHeap<Rule>
}

impl Ruleset {
    /// Construct a new `Ruleset` with a default `NoneTransformer` and `TrimTransformer` global
    /// rules.
    ///
    /// The default trim and none rules should be appropriate for most CSV files. For CSV files
    /// where these default rules are not desired use the `Ruleset::without_default_rules` method.
    pub fn new() -> Ruleset {
        let mut ruleset = Self::without_default_rules();
        // Add a default trim rule and blank rule to match empty fields.
        ruleset.add_rule(Rule::global_with_priority(Transformers::None(NoneTransformer::with_blank_matcher()), -10));
        ruleset.add_rule(Rule::global_with_priority(Transformers::Trim(TrimTransformer::new()), -10));
        ruleset
    }

    /// Construct a new `Ruleset` without any of the default rules.
    pub fn without_default_rules() -> Ruleset {
        Ruleset {
            rules: BinaryHeap::new()
        }
    }

    /// Add a `Rule` to the this ruleset.
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    /// Validate this ruleset against a CSV file by comparing it's `Rule`s against the headers.
    pub fn validate_rules(&self, headers: &Vec<String>) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        for rule in self.rules.iter() {
            if let Applicability::Fields { ref field_names } = rule.applicability {
                let header_set = HashSet::<String>::from_iter(headers.clone());
                let field_set = HashSet::<String>::from_iter(field_names.clone());
                let diff: HashSet<String> = field_set.difference(&header_set).cloned().collect();
                if diff.len() > 0 {
                    // FIXME: We should have a better way to construct a ruleset that uses Result
                    //   instead of panic! here.
                    errors.push(
                        ValidationError {
                            reason: format!("The following fields were not found in headers: '{:?}'", diff),
                        }
                    )
                }
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Apply this `Ruleset` to a record from a CSV file.
    pub fn apply_rules(&self, headers: &Vec<String>, fields: &Vec<String>, record_n: usize) -> TransformedRecord {
        let expected_n_fields = headers.len();

        let mut errors: Vec<TransformError> = Vec::new();
        let mut transformed_fields: Vec<Option<String>> = Vec::new();
        for (field_n, field_value) in fields.iter().enumerate() {
            if field_n < expected_n_fields {
                let field_name = &headers[field_n];
                let mut transformed_field_value = Some(field_value.clone());
                // Try each rule in order of priority and test to see if it is applicable.
                for rule in self.rules.iter() {
                    let new_value = match transformed_field_value {
                        Some(ref fv) => {
                            let transform_result = rule.apply(fv, &field_name, record_n);
                            match transform_result {
                                Ok(tfv) => tfv,
                                Err(e) => {
                                    errors.push(e);
                                    None
                                }
                            }
                        },
                        // The last transformer returned None, so we can short circuit and just
                        // return None for the field value.
                        None => break
                    };
                    transformed_field_value = new_value;
                }
                transformed_fields.insert(field_n, transformed_field_value);
            } else {
                errors.push(
                    TransformError {
                        field_value: field_value.to_string(),
                        field_name: field_n.to_string(),
                        record_n: record_n,
                        reason: format!("found {} header fields but record had extra field at position {}", expected_n_fields, field_n)
                    }
                );
            }
        }

        TransformedRecord {
            field_values: transformed_fields,
            errors: errors,
        }
    }
}

/// Error for when a `Ruleset` does not validate against a CSV file.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ValidationError {
    reason: String,
}

impl Display for ValidationError
{
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{}", self.reason)
    }
}

impl error::Error for ValidationError
{
    fn description(&self) -> &str {
        &self.reason
    }
}

/// A single processed and transformed record.
#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Debug)]
pub struct TransformedRecord {
    /// Transformed fields for the record.
    ///
    /// Empty field are explicitly encoded as `None` values.
    pub field_values: Vec<Option<String>>,
    /// Errors that were encountered during transformation, if any.
    pub errors: Vec<TransformError>,
}
