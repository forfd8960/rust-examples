use std::{collections::HashMap, fs};
use anyhow::Result;

#[derive(Debug)]
pub struct Query {
    pattern: String,
    file_name: String, 
}

pub trait Match {
    // get_matched_lines return matched file name, and lines
    fn get_matched_lines(&self) -> Result<HashMap<String, Vec<String>>>;
}

impl Query {
    pub fn new(p: String, f: String) -> Self {
        Self { pattern: p, file_name: f }
    }
}

impl Match for Query {
    fn get_matched_lines(&self) -> Result<HashMap<String, Vec<String>>> {
        let lines = fs::read_to_string(&self.file_name).expect("unable read file");
        println!("lines: {}", lines);
        let m: HashMap<String, Vec<String>> = HashMap::new();
        Ok(m)
    }
}