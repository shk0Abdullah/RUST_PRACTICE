#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: &'a str,
    delimeter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimeter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimeter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimeter) {
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[next_delim + self.delimeter.len()..];
            Some(until_delimiter)
        } else if self.remainder.is_empty() {
            None
        } else {
            let rest = self.remainder;
            self.remainder = "";
            Some(rest)
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e f";
    let letters: Vec<&str> = StrSplit::new(haystack, " ").collect();
    assert!(letters.eq(&vec!["a", "b", "c", "d", "e", "f"]))
    // assert_eq!(letters.next(), Some("a"));
}
