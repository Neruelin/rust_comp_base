#![allow(unused_imports, non_snake_case, unused)]
use std::io::{BufWriter, BufRead, stdin, stdout, Write};

struct Solution;
impl Solution {
    pub fn solve<T: Write, R: BufRead>(out_stream: &mut Box<T>, mut read_stream: R) {
        let mut scan = Scanner::new(read_stream);
        let out = &mut BufWriter::new(out_stream.as_mut());

        // let read = scan.token::<u64>();

        // writeln!(out, "{}", "").expect("Failed to write");
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn t1() {
        let mut output = Box::new(Vec::with_capacity(128));
        let input = r#""#.as_bytes();

        Solution::solve(&mut output, input);

        let output_txt = String::from_utf8(*output).expect("Failed to convert back to string");

        assert_eq!(output_txt, r#""#);
    }

}

// Boilerplate

pub struct Scanner<B> {
    reader: B,
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitWhitespace<'static>,
}
impl<B: BufRead> Scanner<B> {
    pub fn new(reader: B) -> Self {
        Self {
            reader,
            buf_str: Vec::new(),
            buf_iter: "".split_whitespace()
        }
    }
    pub fn token<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader.read_until(b'\n', &mut self.buf_str).expect("Failed read");
            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace()) }
        }
    }
}

fn main() {
    Solution::solve(&mut Box::new(stdout()), stdin().lock());
}