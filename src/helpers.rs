use std::{fmt::Debug, iter::Sum, str::FromStr};

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

pub fn sum_lines<T>(lines: &str) -> T
where
    T: FromStr + Sum,
    <T as FromStr>::Err: Debug,
{
    lines
        .split('\n')
        .map(|x| x.parse::<T>().unwrap())
        .into_iter()
        .sum()
}
