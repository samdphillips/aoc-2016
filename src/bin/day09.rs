
use std::cmp::min;
use std::io::{BufRead, Read, Result};

fn is_whitespace(c: u8) -> bool {
    c == b' ' || c == b'\n'
}

pub struct FilterWSReader<I> {
    inner: I,
}

impl<I> FilterWSReader<I>
    where I: BufRead
{
    fn new(inner: I) -> FilterWSReader<I> {
        FilterWSReader {
            inner: inner,
        }
    }

    fn skip_spaces(&mut self) -> Result<()> {
        let size = {
            let buf = try!(self.inner.fill_buf());
            buf.iter().take_while(|&&c| is_whitespace(c)).count()
        };
        self.inner.consume(size);
        Ok(())
    }
}

impl<I> Read for FilterWSReader<I>
    where I: BufRead
{
    fn read(&mut self, mut outbuf: &mut [u8]) -> Result<usize> {
        try!(self.skip_spaces());

        let size = {
            let buf = try!(self.inner.fill_buf());
            let size = match buf.iter().position(|&c| is_whitespace(c)) {
                Some(pos) => min(pos, outbuf.len()),
                None => min(buf.len(), outbuf.len()),
            };
            try!(buf[0..size].as_ref().read(&mut outbuf))
        };
        self.inner.consume(size + 1);
        Ok(size)
    }
}



fn main() { }

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Read};

    #[test]
    fn aoc09_test_filter_ws_simple() {
        let x: Vec<u8> = b"abc def abc".to_vec();
        let inner = Cursor::new(x);
        let mut r = FilterWSReader::new(inner);
        let mut s = String::new();
        r.read_to_string(&mut s).unwrap();
        assert_eq!(s.as_str(), "abcdefabc")
    }

    #[test]
    fn aoc09_test_filter_ws_start() {
        let x: Vec<u8> = b" abc def abc".to_vec();
        let inner = Cursor::new(x);
        let mut r = FilterWSReader::new(inner);
        let mut s = String::new();
        r.read_to_string(&mut s).unwrap();
        assert_eq!(s.as_str(), "abcdefabc")
    }

    #[test]
    fn aoc09_test_filter_ws_extra() {
        let x: Vec<u8> = b" abc def \n   abc".to_vec();
        let inner = Cursor::new(x);
        let mut r = FilterWSReader::new(inner);
        let mut s = String::new();
        r.read_to_string(&mut s).unwrap();
        assert_eq!(s.as_str(), "abcdefabc")
    }
}
