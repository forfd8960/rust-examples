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
        let content = fs::read_to_string(&self.file_name).expect("unable read file");
        let mut m: HashMap<String, Vec<String>> = HashMap::new();
        let lines = content.split("\n");

        let mut value: Vec<String> = Vec::new();
        for line in lines {
            value.push(String::from(line));
        }
        m.insert(self.file_name.clone(), value);

        Ok(m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_matched_lines() {
        let f = String::from("test.txt");
        let file = f.clone();
        let q = Query::new("abc".into(), f);
        let result = match q.get_matched_lines() {
            Ok(v) => v,
            Err(e) => {
                println!("has err: {:?}", e);
                let res = HashMap::new();
                res
            }
        };

        let mut expect_result: HashMap<String, Vec<String>> = HashMap::new();
        let lines : Vec<String>  = vec![
            "abc".into(),
            "test".into(),
            "xyz".into(),
        ];
        expect_result.insert(file, lines);
        assert_eq!(result, expect_result);
    }
}