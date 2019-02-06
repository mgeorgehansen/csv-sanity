use std::hash::{
    Hash,
    Hasher,
};
use regex;
use serde::{
    Serialize,
    Serializer,
    Deserialize,
    Deserializer,
};

custom_derive! {
    #[derive(NewtypeFrom, NewtypeDeref, NewtypeDerefMut, Clone, NewtypeDisplay, NewtypeDebug)]
    pub struct Regex(regex::Regex);
}

impl PartialEq for Regex {
    fn eq(&self, other: &Regex) -> bool
    {
        self.0.as_str() == other.0.as_str()
    }
}

impl Eq for Regex {}

impl Hash for Regex {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher {
        self.as_str().hash(state);
    }
}

impl Serialize for Regex
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where S: Serializer {
        let Regex(ref regex) = *self;
        regex.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Regex
{
    fn deserialize<D>(deserializer: D) -> Result<Regex, D::Error>
      where D: Deserializer<'de>
    {
        use serde::de::{Unexpected, Error};
        let string: Result<String, D::Error> = Deserialize::deserialize(deserializer);
        string.and_then(|s| {
            regex::Regex::new(&s)
                .map(|r| Regex(r))
                .map_err(|e| {
                    let message: &str = &format!("invalid regex string: {}", e);
                    D::Error::invalid_value(Unexpected::Str(&s), &message)
            })
        })
    }
}
