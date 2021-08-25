/*

        LeTo - Lexer and Tokenizer
            by Zac Reid 

        byte2char - V0.1.0 - 15/08/2021

*/

use std::char;

pub struct Byte2Char<'a> {
    input: &'a [u8],
    length: usize,
    pos: usize,
}

impl<'a> Byte2Char<'a> {
    #[inline(always)]
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            input: input,
            length: input.len(),
            pos: 0,
        }
    }
}

impl<'a> Iterator for Byte2Char<'a> {
    type Item = char;

    #[inline(always)]
    fn next(&mut self) -> Option<char> {

        if self.length <= self.pos {
            return None;
        }

        let current_byte = self.input[self.pos];

         if current_byte < 0xC0 {
            self.pos += 1;
            
            return Some((current_byte & 0x7F) as char)
        }else if current_byte < 0xE0 {
            let second_byte:u32 = self.input[self.pos + 1] as u32;
            self.pos += 2;

            return char::from_u32((((current_byte as u32) & 0x1F) << 6) | (second_byte & 0x3F))
        } else if current_byte < 0xF0 {
            let second_byte:u32 = self.input[self.pos + 1] as u32;
            let third_byte:u32 = self.input[self.pos + 2] as u32;

            self.pos += 3;

            return char::from_u32((((current_byte as u32) & 0xF) << 12) | ((second_byte & 0x3F) << 6) | (third_byte & 0x3F))
        } else {
            let second_byte:u32 = self.input[self.pos + 1] as u32;
            let third_byte:u32 = self.input[self.pos + 2] as u32;
            let fourth_byte:u32 = self.input[self.pos + 3] as u32;

            self.pos += 4;

            return char::from_u32((((current_byte as u32) & 0x7) << 18) | ((second_byte & 0x3F) << 12) | ((third_byte & 0x3F) << 6) | (fourth_byte & 0x3F))
        }
    }
}