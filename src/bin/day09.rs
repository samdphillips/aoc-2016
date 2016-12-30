
use std::iter::Chain;

#[derive(Debug)]
struct Repeater {
    repeat: usize,
    pos: usize,
    elems: Vec<u8>
}

fn repeater<I>(repeat: usize, it: I) -> Repeater
    where I: IntoIterator<Item = u8> {
        Repeater { repeat: repeat, pos: 0, elems: it.into_iter().collect() }
}

impl Iterator for Repeater {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.pos == self.elems.len() {
            self.pos = 0;
            self.repeat = self.repeat - 1
        }

        if self.repeat == 0 {
            return None
        }

        let v = self.elems[self.pos];
        self.pos = self.pos + 1;
        Some(v)
    }
}

fn decompress<I>(count: usize, repeat: usize, i: I) -> Chain<Repeater, <I as std::iter::IntoIterator>::IntoIter>
    where I: IntoIterator<Item=u8> {
        let mut it = i.into_iter();
        repeater(repeat, it.by_ref().take(count)).chain(it)
}

#[test]
fn aoc09_test_decompress() {
    let v = b"abcdef".into_iter().cloned();
    assert_eq!(decompress(3, 3, v).collect::<Vec<u8>>(), b"abcabcabcdef".to_vec());
}

#[test]
fn aoc09_test_parse() {
}

fn main() { }
