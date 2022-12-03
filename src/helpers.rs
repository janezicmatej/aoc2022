use lazy_static::lazy_static;
use std::{fmt::Debug, str::FromStr};

pub fn to_vec<T>(input: &str, split: char) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input
        .split(split)
        .map(|x| x.parse::<T>().unwrap())
        .collect()
}

pub fn to_vec_vec<T>(input: &str, split_rows: char, split_row: char) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input
        .split(split_rows)
        .map(|x| {
            x.split(split_row)
                .map(|y| y.parse::<T>().unwrap())
                .collect()
        })
        .collect()
}

lazy_static! {
    pub static ref ASCII_LOWERCASE: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    pub static ref ASCII_UPPERCASE: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    pub static ref ASCII: [char; 52] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
}
