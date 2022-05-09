#![feature(generators, generator_trait)]

use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign};

pub mod gen;
mod num;

pub use num::Int;

#[derive(Clone, Debug, Eq)]
pub struct Digits(pub usize, pub Vec<usize>);

impl PartialEq<Int> for Digits {
    fn eq(&self, rhs: &Int) -> bool {
        self.0 == rhs.base && self.1 == rhs.digs
    }
}

#[derive(Clone, Debug)]
pub struct UniqDigits(Digits, HashSet<Digits>);

impl UniqDigits {
    pub fn new(base: usize, len: usize) -> Self {
        UniqDigits(Digits::new(base, len), HashSet::new())
    }
}

use gen_iter::gen_iter;

pub struct Iter {
    int: Int,
    iter: Box<dyn Iterator<Item = usize>>,
}

impl Iter {
    pub fn new(bw: usize) -> Self {
        Iter {
            int: Int::new(bw),
            iter: Box::new(IncIter::new(bw, 2)),
        }
    }
}

impl Iterator for Iter {
    type Item = Int;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.int.clone();
        self.int += self.iter.next().unwrap();
        Some(i)
    }
}

pub struct IncIter;

impl IncIter {
    pub fn new(n: usize, bj: usize) -> impl Iterator<Item = usize> {
        gen_iter!(move {
                for b in bj.. {
                    match (n, b) {
                        (_, bl) if bj == bl => {
                            for _ in 0..(bl.pow(n as _) - 1) {
                                yield 1;
                            }
                        }
                        (2, b) => {
                            for i in 0..(b - 1 + b) {
                                if i < b - 1 {
                                    yield b;
                                } else {
                                    yield 1;
                                }
                            }
                        }
                        (n, b) => {
                            let skip = b - 1;
                            let pass = b;
                            let mut len = Vec::with_capacity(n - 2);
                            len.insert(0, skip + pass);
                            for i in 1..n - 2 {
                                len.insert(i, skip * len[i - 1] + usize::pow(b, (i + 1) as _));
                            }
                            for i in 0..(skip * len[len.len() - 1]) {
                                let mut j = 0;
                                yield loop {
                                    let mut i = i;
                                    for k in (j..len.len()).rev() {
                                        i %= len[k];
                                    }
                                    if i >= skip * if j == 0 { 1 } else { len[j - 1] } {
                                        break 1;
                                    }
                                    if j == len.len() {
                                        break b;
                                    }
                                    j += 1;
                                }
                            }
                            for _ in 0..usize::pow(b, (n - 1) as _) {
                                yield 1;
                            }
                        }
                    }
                }
        })
    }
}

impl Iterator for UniqDigits {
    type Item = Digits;

    fn next(&mut self) -> Option<Self::Item> {
        let mut d = self.0.next().unwrap();
        while self.1.contains(&d) {
            d = self.0.next().unwrap();
        }
        self.1.insert(d.clone());
        Some(d)
    }
}

pub struct SeqDigits(Box<dyn Iterator<Item = usize>>, Digits);

impl Iterator for SeqDigits {
    type Item = Digits;

    fn next(&mut self) -> Option<Self::Item> {
        let (s, mut d) = (self.0.next().unwrap(), self.1.next().unwrap());
        for _ in 0..s - 1 {
            d = self.1.next().unwrap();
        }
        Some(d)
    }
}

impl Digits {
    pub fn new(base: usize, len: usize) -> Self {
        Digits(base, vec![0; len])
    }

    fn inc(&mut self) {
        self.1[0] += 1;
        for i in 0..self.1.len() {
            if self.1[i] == self.0 {
                self.1[i] = 0;
                if i + 1 < self.1.len() {
                    self.1[i + 1] += 1;
                } else {
                    self.0 += 1;
                }
            }
        }
    }
}

impl AddAssign<usize> for Digits {
    fn add_assign(&mut self, mut rhs: usize) {
        log::debug!("{:?} + {}", self, rhs);
        'outer: loop {
            for i in 0..self.1.len() {
                log::debug!("{:?}", self);
                let cry = (self.1[i] + rhs) / self.0;
                log::debug!("rhs={rhs} cry={cry}");
                log::debug!(
                    "set self.1[{i}] = ({} + {} % {} = {}",
                    self.1[i],
                    rhs,
                    self.0,
                    (self.1[i] + rhs) % self.0
                );
                self.1[i] = (self.1[i] + rhs) % self.0;
                let len = self.1.len();
                self.1[(i + 1) % len] += cry;
                log::debug!("{:?}", self);

                rhs = match rhs {
                    0 => break 'outer,
                    _ if rhs <= self.0 => 0,
                    _ => rhs % self.0,
                };
            }
        }
    }
}

impl Add<usize> for Digits {
    type Output = Self;

    fn add(mut self, rhs: usize) -> Self::Output {
        self += rhs;
        self
    }
}

impl Hash for Digits {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.1.hash(state);
    }
}

impl PartialEq for Digits {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl Iterator for Digits {
    type Item = Digits;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.clone();
        self.inc();
        Some(c)
    }
}

impl Into<usize> for Digits {
    fn into(self) -> usize {
        let mut b = 1;
        let mut p = 0;
        for i in self.1 {
            p += i * b;
            b *= self.0;
        }
        p
    }
}

impl fmt::Display for Digits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const ALPHABET: &[u8; 62] =
            b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

        match self.0 {
            2 => write!(f, "0b"),
            8 => write!(f, "0o"),
            16 => write!(f, "0x"),
            r => write!(f, "{}r", r),
        }?;
        for c in &self.1 {
            write!(f, "{}", ALPHABET[*c] as char)?;
        }
        Ok(())
    }
}

#[test]
fn test_digits_inc() {
    fn inc(mut d: Digits) -> Digits {
        d.inc();
        d
    }
    assert_eq!(inc(Digits(2, vec![0, 0])), Digits(2, vec![1, 0]));
    assert_eq!(inc(Digits(2, vec![1, 0])), Digits(2, vec![0, 1]));
    assert_eq!(inc(Digits(2, vec![0, 1])), Digits(2, vec![1, 1]));
    assert_eq!(inc(Digits(2, vec![1, 1])), Digits(3, vec![0, 0]));
}

#[test]
#[ignore]
fn test_digits_add() {
    env_logger::try_init().unwrap();

    assert_eq!(Digits::new(2, 2) + 0, Digits(2, vec![0, 0]));
    assert_eq!(Digits::new(2, 2) + 1, Digits(2, vec![1, 0]));
    assert_eq!(Digits::new(2, 2) + 2, Digits(2, vec![0, 1]));
    assert_eq!(Digits::new(2, 2) + 3, Digits(2, vec![1, 1]));
    assert_eq!(Digits::new(2, 2) + 4, Digits(3, vec![0, 0]));
}

#[test]
fn test_digits_iter() {
    assert_eq!(
        Digits::new(2, 2).take(8).collect::<Vec<_>>(),
        vec![
            Digits(2, vec![0, 0]),
            Digits(2, vec![1, 0]),
            Digits(2, vec![0, 1]),
            Digits(2, vec![1, 1]),
            Digits(3, vec![0, 0]),
            Digits(3, vec![1, 0]),
            Digits(3, vec![2, 0]),
            Digits(3, vec![0, 1]),
        ],
    );
}

#[test]
fn test_digits_into() {
    assert_eq!(<Digits as Into<usize>>::into(Digits(2, vec![1, 1])), 3);
    assert_eq!(<Digits as Into<usize>>::into(Digits(3, vec![1, 0, 1])), 10);
    assert_eq!(<Digits as Into<usize>>::into(Digits(5, vec![0, 1, 1])), 30);
}
