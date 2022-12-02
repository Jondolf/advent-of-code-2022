use std::{fmt::Debug, str::FromStr};

/// Parses a string iterator's strings into a given type
pub fn parse_strings<'a, I, T>(items: I) -> Vec<T>
where
    I: Iterator<Item = &'a str>,
    T: FromStr,
    <T as std::str::FromStr>::Err: Debug,
{
    items.map(|item| item.parse::<T>().unwrap()).collect()
}
