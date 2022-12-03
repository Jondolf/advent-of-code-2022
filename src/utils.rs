use std::{collections::BTreeSet, fmt::Debug, str::FromStr};

/// Parses a string iterator's strings into a given type
pub fn parse_strings<'a, I, T>(items: I) -> Vec<T>
where
    I: Iterator<Item = &'a str>,
    T: FromStr,
    <T as std::str::FromStr>::Err: Debug,
{
    items.map(|item| item.parse::<T>().unwrap()).collect()
}

pub fn intersection<'a, I, T>(mut sets: I) -> BTreeSet<T>
where
    I: Iterator<Item = &'a BTreeSet<T>>,
    T: Clone + Ord + 'a,
{
    sets.next()
        .map(|set| sets.fold(set.to_owned(), |set1, set2| (&set1 & set2)))
        .unwrap()
}
