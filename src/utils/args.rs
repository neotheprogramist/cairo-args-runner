use cairo_felt::Felt252;
use cairo_lang_runner::Arg;
use serde::{de::Visitor, Deserialize};
use serde_json::Value;
use std::ops::Deref;

#[derive(Debug)]
pub struct WrappedArgs(Vec<Arg>);
impl WrappedArgs {
    pub fn new(args: Vec<Arg>) -> Self {
        Self(args)
    }
}
impl Deref for WrappedArgs {
    type Target = Vec<Arg>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<WrappedArgs> for Vec<Arg> {
    fn from(args: WrappedArgs) -> Self {
        args.0
    }
}
impl From<Vec<Arg>> for WrappedArgs {
    fn from(args: Vec<Arg>) -> Self {
        Self(args)
    }
}

impl<'de> Visitor<'de> for WrappedArgs {
    type Value = WrappedArgs;
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
                    args.push(Arg::Array(inner_args))
                }
                _ => return Err(serde::de::Error::custom("Invalid type")),
            }
        }
        Ok(WrappedArgs::new(args))
    }
}
impl<'de> Deserialize<'de> for WrappedArgs {
    fn deserialize<D>(deserializer: D) -> Result<WrappedArgs, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(WrappedArgs(Vec::new()))
    }
}