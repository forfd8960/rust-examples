#[derive(Debug)]
struct StrSplit<'a, TK> {
    remain: Option<&'a str>,
    token: TK
}

impl<'a, TK> StrSplit<'a, TK> {
    pub fn new(s: &'a str, token: TK) -> Self {
        Self {
            remain: Some(s),
            token
        }
    }
}

impl<'a, TK> Iterator for StrSplit<'a, TK> where TK: Tokenier {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let rm = self.remain.as_mut()?;
        if let Some((tk_start, tk_end)) = self.token.find_next(rm) {
            let until = &rm[..tk_start];
            *rm = &rm[tk_end..];
            Some(until)
        } else {
            self.remain.take()
        }
    }
}

pub trait Tokenier {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl Tokenier for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices().find(|(_, c)| c == self).map(|(start, _)| (start, start + self.len_utf8()))
    }
}


fn main() {
    let s = "hello, rust, and, good, morning";
    let ss = StrSplit::new(&s, ',');
    println!("strsplit: {:?}", ss);
    let words: Vec<_> = ss.collect();
    println!("words: {:?}", words);
    assert_eq!(words, vec!["hello", " rust", " and", " good", " morning"])
}