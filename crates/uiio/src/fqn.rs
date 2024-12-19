use nom::{
    bytes::complete::{tag, take_while1},
    character::is_alphanumeric,
    multi::separated_list1,
    IResult,
};
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;

/// Helper function to parse a valid identifier component
fn identifier(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| is_alphanumeric(c as u8) || c == '_')(input)
}

/// Function to parse an FQN
fn fqn(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag("."), identifier)(input)
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse FQN: {0}")]
    FailedToParse(String),
    #[error("Unexpected input remaining: {0}")]
    Remaining(String),
}

#[derive(Debug, PartialEq)]
pub struct Fqn {
    path: Vec<Arc<String>>,
}

impl FromStr for Fqn {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match fqn(s) {
            Ok((remaining, components)) if remaining.is_empty() => {
                let path = components
                    .into_iter()
                    .map(String::from)
                    .map(Arc::new)
                    .collect();
                Ok(Fqn { path })
            }
            Ok((remaining, _)) => Err(Error::Remaining(remaining.into())),
            Err(err) => Err(Error::FailedToParse(err.to_string())),
        }
    }
}
