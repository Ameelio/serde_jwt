use serde::{Deserialize, Serialize};

/// Represents a deserialized value that can legally be either a single string or
/// an array of strings.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum OneOrMore {
    One(Box<str>),
    More(Box<[Box<str>]>),
}

pub struct OneOrMoreIterator<'a> {
    inner: &'a OneOrMore,
    index: usize,
}

impl OneOrMore {
    /// This iterates over &str, in the case of `OneOrMore::One`, it will
    /// return the value if index is 0, and `Option::None` otherwise.
    pub fn iter(&self) -> OneOrMoreIterator<'_> {
        OneOrMoreIterator {
            inner: self,
            index: 0,
        }
    }
}

impl<'a> IntoIterator for &'a OneOrMore {
    type Item = &'a str;
    type IntoIter = OneOrMoreIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> Iterator for OneOrMoreIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let len: usize = match self.inner {
            OneOrMore::One(_) => 1,
            OneOrMore::More(list) => list.len(),
        };

        if self.index < len {
            let result: Self::Item = match self.inner {
                OneOrMore::One(one) => one,
                OneOrMore::More(more) => (more[self.index]).as_ref(),
            };

            self.index += 1;

            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_will_match_any_against_one() {
        let v1 = OneOrMore::One("one".into());
        let v2 = {
            let v: Box<[Box<str>]> = ["one".into(), "two".into()].into();

            OneOrMore::More(v)
        };

        assert!(v1.iter().any(|x| x == "one"));
        assert!(v2.iter().any(|x| x == "one"));
        assert!(v2.iter().any(|x| x == "two"));
        assert_eq!(v1.iter().any(|x| x == "two"), false);
    }
}
