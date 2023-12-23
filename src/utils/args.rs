use std::ops::Deref;

use cairo_felt::Felt252;
use cairo_lang_runner::Arg;
use serde::{de::Visitor, Deserialize};
use serde_json::Value;

/// `WrappedArg` is a wrapper around a vector of `Arg`.
///
/// It provides convenience methods for working with a vector of `Arg` and implements
/// `Deref` to allow it to be treated like a vector of `Arg`.
#[derive(Debug)]
pub struct WrappedArg(Vec<Arg>);

impl WrappedArg {
    /// Creates a new `WrappedArg` from a vector of `Arg`.
    ///
    /// # Arguments
    ///
    /// * `args` - A vector of `Arg`.
    ///
    /// # Returns
    ///
    /// * `WrappedArg` - A new `WrappedArg` instance.
    #[must_use]
    pub fn new(args: Vec<Arg>) -> Self {
        Self(args)
    }
}

impl Deref for WrappedArg {
    type Target = Vec<Arg>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<WrappedArg> for Vec<Arg> {
    fn from(args: WrappedArg) -> Self {
        args.0
    }
}

impl From<Vec<Arg>> for WrappedArg {
    fn from(args: Vec<Arg>) -> Self {
        Self(args)
    }
}

impl<'de> Visitor<'de> for WrappedArg {
    type Value = WrappedArg;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a list of arguments")
    }
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut args = Vec::new();
        while let Some(arg) = seq.next_element()? {
            match arg {
                Value::Number(n) => args.push(Arg::Value(Felt252::from(n.as_u64().unwrap()))),
                Value::Array(a) => {
                    let mut inner_args = Vec::new();
                    for n in a {
                        match n {
                            Value::Number(n) => inner_args.push(Felt252::from(n.as_u64().unwrap())),
                            _ => return Err(serde::de::Error::custom("Invalid type")),
                        }
                    }
                    args.push(Arg::Array(inner_args));
                }
                _ => return Err(serde::de::Error::custom("Invalid type")),
            }
        }
        Ok(WrappedArg::new(args))
    }
}

impl<'de> Deserialize<'de> for WrappedArg {
    fn deserialize<D>(deserializer: D) -> Result<WrappedArg, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(WrappedArg(Vec::new()))
    }
}
