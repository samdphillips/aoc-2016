
use std::cmp::min;
use std::io::{BufRead, BufReader, Cursor, Read, Result};

fn is_whitespace(c: u8) -> bool {
    c == b' ' || c == b'\n'
}

struct FilterSpace<I> {
    inner: I,
}

impl<I> FilterSpace<I>
    where I: BufRead
{
    fn new(inner: I) -> FilterSpace<I> {
        FilterSpace {
            inner: inner,
        }
    }

    fn skip_spaces(&mut self) -> Result<()> {
        let size =
            {
                let buf = try!(self.inner.fill_buf());
                buf.iter().take_while(|&&c| is_whitespace(c)).count()
            };
        self.inner.consume(size);
        Ok(())
    }

    fn into_bufreader(self) -> BufReader<Self> {
        BufReader::new(self)
    }
}

impl<I> Read for FilterSpace<I>
    where I: BufRead
{
    fn read(&mut self, mut outbuf: &mut [u8]) -> Result<usize> {
        try!(self.skip_spaces());

        let size =
            {
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

#[test]
fn aoc09_test_filter_spaces_simple() {
    let x: Vec<u8> = b"abc def abc".to_vec();
    let inner = Cursor::new(x);
    let mut r = FilterSpace::new(inner);
    let mut s = String::new();
    r.read_to_string(&mut s).unwrap();
    assert_eq!(s.as_str(), "abcdefabc")
}

#[test]
fn aoc09_test_filter_spaces_start() {
    let x: Vec<u8> = b" abc def abc".to_vec();
    let inner = Cursor::new(x);
    let mut r = FilterSpace::new(inner);
    let mut s = String::new();
    r.read_to_string(&mut s).unwrap();
    assert_eq!(s.as_str(), "abcdefabc")
}

#[test]
fn aoc09_test_filter_spaces_extra() {
    let x: Vec<u8> = b" abc def \n   abc".to_vec();
    let inner = Cursor::new(x);
    let mut r = FilterSpace::new(inner);
    let mut s = String::new();
    r.read_to_string(&mut s).unwrap();
    assert_eq!(s.as_str(), "abcdefabc")
}

fn main() { }
