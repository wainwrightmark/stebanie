use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Puzzle {
    pub title: String,
    pub map: String,
    pub location: Location,
    pub html: String,
    pub questions: Vec<Question>,
}

impl Puzzle {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Location {
    latitude: f64,
    longitude: f64,
}

impl Location {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Question {
    pub q: String,
    pub a: String,
    pub length: Option<usize>,
}

impl Question {
    // pub fn is_correct_answer(&self, text: &str)-> bool{
    //     text.trim().eq_ignore_ascii_case(self.a.as_str().trim())
    // }

    pub fn placeholder(&self) -> String {
        let length = self.length.unwrap_or(self.a.len());
        "_ ".repeat(length)
    }

    pub fn pattern(&self) -> String {
        let mut output = "".to_string();
        for c in self.a.chars() {
            if c.is_ascii_alphabetic() {
                output.push('[');
                output.push(c.to_ascii_uppercase());
                output.push(c.to_ascii_lowercase());
                output.push(']');
            } else {
                output.push(c)
            }
        }
        output
    }

    pub fn min_length(&self) -> usize {
        self.length.unwrap_or(self.a.len())
    }

    pub fn max_length(&self) -> usize {
        self.length.unwrap_or(self.a.len())
    }
}

lazy_static::lazy_static! {
    pub static ref PUZZLES: Vec<Puzzle> ={
        let s = include_str!("puzzles.yaml");
        let list: Vec<Puzzle> = serde_yaml::from_str(s).expect("Could not deserialize list of puzzles");

        list
    };
}

#[cfg(test)]
mod tests {
    use crate::puzzle::*;

    #[test]
    pub fn test_campaign_levels_deserialize() {
        let list = &PUZZLES;
        assert!(list.len() > 0)
    }
}
