use std::{borrow::Cow, cmp::Ordering};

#[derive(Debug)]
pub(crate) enum Options {
    Number,
    Revers,
    NonRepeating,
    Month,
    IsSorter,
    Suffix,
}

pub struct Month(usize);

impl TryFrom<&str> for Month {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "january" | "jan" => Ok(Month(1)),
            "february" | "feb" => Ok(Month(2)),
            "march" | "mar" => Ok(Month(3)),
            "april" | "apr" => Ok(Month(4)),
            "may" => Ok(Month(5)),
            "june" | "jun" => Ok(Month(6)),
            "july" | "jul" => Ok(Month(7)),
            "august" | "aug" => Ok(Month(8)),
            "september" | "sep" => Ok(Month(9)),
            "october" | "oct" => Ok(Month(10)),
            "november" | "nov" => Ok(Month(11)),
            "december" | "dec" => Ok(Month(12)),
            _ => Err("Cannot parse to Month".to_owned()),
        }
    }
}

impl ToString for Month {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl TryFrom<Month> for String {
    type Error = String;
    fn try_from(value: Month) -> Result<Self, Self::Error> {
        match value.0 {
            1 => Ok("january".to_string()),
            2 => Ok("february".to_owned()),
            3 => Ok("march".to_owned()),
            4 => Ok("april".to_owned()),
            5 => Ok("may".to_owned()),
            6 => Ok("june".to_owned()),
            7 => Ok("july".to_owned()),
            8 => Ok("august".to_owned()),
            9 => Ok("september".to_owned()),
            10 => Ok("october".to_owned()),
            11 => Ok("november".to_owned()),
            12 => Ok("december".to_owned()),
            _ => Err("Cannot convert from Month".to_owned()),
        }
    }
}

pub fn is_sort_num(data: &[f64]) -> bool {
    data.windows(2).all(|w| w[0] <= w[1])
}
pub fn is_sort_string(data: &[String], whitespace: bool) -> bool {
    data.windows(2).all(|w| {
        if !whitespace {
            w[0] <= w[1]
        } else {
            w[0].trim() <= w[1].trim()
        }
    })
}

pub fn sort_number(mut data: Vec<String>) -> Vec<String> {
    let mut middleware = data
        .iter()
        .filter_map(|word| {
            word.parse::<f64>()
                .ok()
                .map(|num| (Cow::Borrowed(word.trim()), num))
        })
        .collect::<Vec<(Cow<'_, str>, f64)>>();
    middleware.sort_by(|a, b| {
        if let Some(ord) = a.1.partial_cmp(&b.1) {
            return ord;
        }
        Ordering::Equal
    });
    data = middleware
        .into_iter()
        .map(|(word_like_num, _)| word_like_num.into_owned())
        .collect();
    data
}
pub fn sort_month(data: Vec<String>) -> Vec<String> {
    let mut middleware = data
        .iter()
        .filter_map(|month| month.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    middleware.sort();
    middleware
        .into_iter()
        .filter_map(|month| {
            let month: Option<String> = Month(month).try_into().ok();
            month
        })
        .collect()
}

pub fn get_words(data: Vec<String>, opts: (usize, bool)) -> Vec<String> {
    let data = data.into_iter().fold(Vec::new(), |mut vault, line| {
        let word = if opts.1 {
            line.split_whitespace()
                .nth(opts.0)
                .map(|s| s.trim().to_string())
        } else {
            let line: Vec<&str> = line.split(' ').collect();

            let mut word = String::new();
            let mut start = 0;
            let mut end = 0;
            let mut word_index = 0;
            let mut leading_spaces = 0;
            for data in line.iter() {
                if !data.is_empty() {
                    if word_index == opts.0 {
                        word.push_str(&" ".repeat(leading_spaces));
                        word.push_str(data);
                        leading_spaces = 0;
                    }
                    word_index += 1;
                    end = start + data.len();
                } else if word_index == opts.0 {
                    leading_spaces += 1;
                }
                start = end + 1;
            }
            if word_index == opts.0 {
                let trailing_spaces = line.len() - end;
                word.push_str(&" ".repeat(trailing_spaces));
            }

            if word.is_empty() {
                None
            } else {
                Some(word)
            }
        };
        if let Some(word) = word {
            vault.push(word);
        }
        vault
    });
    data
}
