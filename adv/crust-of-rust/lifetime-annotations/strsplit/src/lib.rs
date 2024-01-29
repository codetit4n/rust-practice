//#![warn(missing_debug_implementations, missing_docs)]

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
            let until_delimeter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimeter.len())..];
            Some(until_delimeter)
        } else if self.remainder.is_empty() {
            // TODO: bug
            None
        } else {
            let rest = self.remainder;
            self.remainder = "";
            Some(rest)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let haystack = "a b c d e";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e"])
    }

    #[test]
    fn tail() {
        //TODO
        let haystack = "a b c d ";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", ""])
    }
}
