/*

        LeTo - Lexer and Tokenizer
            by Zac Reid 

        byte2char - V0.1.0 - 15/08/2021

*/

pub struct Byte2Char<'a> {
    input: &'a [u8],
    length: usize,
    pos: usize,
}

impl<'a> Byte2Char<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            input: input,
            length: input.len(),
            pos: 0,
        }
    }
}

impl<'a> Iterator for Byte2Char<'a> {
    type Item = [u8;5];

    #[inline]
    fn next(&mut self) -> Option<[u8;5]> {

        if self.length <= self.pos {
            return None;
        }

        let a = self.input[self.pos];

        if a < 0xC0 {
            self.pos += 1;

            return Some([0,0,0,a, 1]);
        } else if a < 0xE0 {
            let b = self.input[self.pos + 1];
            self.pos += 2;

            return Some([0,0,a,b, 2]);
        } else if a < 0xF0 {
            let b = self.input[self.pos + 1];
            let c = self.input[self.pos + 2];
            self.pos += 3;

            return Some([0,a,b,c, 3]);
        } else {
            let b = self.input[self.pos + 1];
            let c = self.input[self.pos + 2];
            let d = self.input[self.pos + 3];
            self.pos += 4;

            return Some([a,b,c,d, 4]);
        }
    }
}