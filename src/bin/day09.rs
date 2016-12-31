
use std::cmp::min;
use std::io::{BufRead, Cursor, Read};

struct Decompress<B> {
    inner: B,
    bufs: Vec<Vec<u8>>,
    offset: usize,
    bytes_ready: usize
}

impl<B> Decompress<B>
    where B: BufRead
{
    fn new(inner: B) -> Decompress<B> {
        Decompress { inner: inner, bufs: Vec::new(), offset: 0, bytes_ready: 0 }
    }

    fn clean_buffers(&mut self) {
        let mut clean = false;

        match self.bufs.last() {
            Some(b) => {
                if self.offset == b.len() {
                    clean = true
                }
            }
            None => ()
        }

        if clean {
            self.bufs.pop();
        }
    }

    fn fill_buffer(&mut self) -> std::io::Result<()> {
        self.clean_buffers();

        if self.bytes_ready == 0 {
            self.decompress()
        } else {
            Ok(())
        }
    }

    fn decompress(&mut self) -> std::io::Result<()> {

        Ok(())
    }
}

impl<B> Read for Decompress<B>
    where B: BufRead
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        try!(self.fill_buffer());
        if self.bytes_ready == 0 {
            return Ok(0)
        }

        let b = self.bufs.last().unwrap();
        let sz = min(self.bytes_ready, b.len());
        try!(b[self.offset..self.offset + sz].as_ref().read(buf));
        self.bytes_ready -= sz;
        self.offset = sz;
        Ok(sz)
    }
}

#[test]
fn aoc09_test_parse() {
    let inp = Cursor::new(b"(3x3)abcdef");
    let mut dec = Decompress::new(inp);
    let mut buf = String::new();
    dec.read_to_string(&mut buf).unwrap();
    println!("read {:?}", buf);
}

fn main() { }
