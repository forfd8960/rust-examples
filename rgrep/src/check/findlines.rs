#[derive(Debug)]
pub struct Query {
    pattern: String,
    file_name: String, 
}

impl Query {
    pub fn new(p: String, f: String) -> Self {
        Self { pattern: p, file_name: f }
    }
}