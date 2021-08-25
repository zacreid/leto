/*

        LeTo - Lexer and Tokenizer
            by Zac Reid 

        char2token - V0.1.0 - 18/08/2021

*/

pub struct Char2Token<'a, Iter> {
    iter: Iter,
    delim: char,
    current_token: std::cell::Cell<String>,
    returned_token: &'a std::cell::Cell<String>,
    at_end: bool,
}

impl<'a, Iter: Iterator<Item=char>> Iterator for Char2Token<'a, Iter> {
    type Item = &'a std::cell::Cell<String>;
    #[inline(always)]
    fn next(&mut self) -> Option<&'a std::cell::Cell<String>> {
        if self.at_end {
            return None
        }

        loop {
            match self.iter.next() {
                Some(c) => {
                    if self.delim == c {
                        self.current_token.swap(self.returned_token);
                        let res = Some(self.returned_token);
                        self.current_token.get_mut().clear();
                        return res;
                    }

                    self.current_token.get_mut().push(c);
                }
                None => {
                    if self.current_token.get_mut().len() > 0 {
                        self.at_end = true;
                        self.current_token.swap(self.returned_token);
                        return Some(self.returned_token);
                    }

                    self.at_end = true;
                    return None;
                }
            }
        }
    }
}

impl<'a, Iter> Char2Token<'a, Iter> {
    #[inline(always)]
    pub fn new(it: Iter, ret_c: &'a std::cell::Cell<String>) -> Self {
        Char2Token {
            iter: it,
            delim: ' ',
            current_token: std::cell::Cell::default(),
            returned_token: ret_c,
            at_end: false,
        }
    }
}