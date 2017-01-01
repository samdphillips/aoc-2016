
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


struct ExtBufReader<R> {
    inner: R,
    buf: Box<[u8]>,
    pos: usize,
    cap: usize,
}

impl<R> ExtBufReader<R>
    where R: Read
{
    fn new(inner: R) -> ExtBufReader<R> {
        ExtBufReader {
            inner: inner,
            buf: vec![0; 16].into_boxed_slice(),
            pos: 0,
            cap: 0
        }
    }

    fn resize_buffer(&mut self, size: usize) -> Result<()> {
        let orig_size = self.cap - self.pos;
        let size = size + orig_size;
        let mut new_buf: Box<[u8]> = vec![0; size].into_boxed_slice();
        try!(self.buf[self.pos..self.cap].as_ref().read(&mut new_buf[size-orig_size..size]));
        self.buf = new_buf;
        self.pos = size - orig_size;
        self.cap = size;
        Ok(())
    }

    fn prepend_buffer(&mut self, mut data: &[u8]) -> Result<()> {
        let data_size = data.len();
        if data_size > self.pos {
            try!(self.resize_buffer(data_size))
        }
        try!(data.read(&mut self.buf[self.pos - data_size..self.pos]));
        self.pos = self.pos - data_size;
        Ok(())
    }
}

#[test]
fn foo() {
    use std::io::Cursor;
    let mut a = ExtBufReader::new(Cursor::new("abc"));
    a.cap = a.inner.read(&mut a.buf).unwrap();
    println!("{:?}", a.buf);
    a.prepend_buffer(b"abcdef").unwrap();
    println!("{:?}", a.buf);
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
