#![feature(generators, generator_trait)]

use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign};

pub mod gen;

#[derive(Clone, Debug, Eq)]
pub struct Digits(pub usize, pub Vec<usize>);

#[derive(Clone, Debug)]
pub struct UniqDigits(Digits, HashSet<Digits>);

impl UniqDigits {
    pub fn new(base: usize, len: usize) -> Self {
        UniqDigits(Digits::new(base, len), HashSet::new())
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

fn seq(n: usize) -> Box<dyn Iterator<Item = usize>> {
    match n {
        0 => unimplemented!(),
        1 => Box::new(std::iter::once(1).chain(3..)),
        2 => Box::new(std::iter::once(1).chain(3..)),
        3 => Box::new(std::iter::once(1).chain(3..)),
        _ => Box::new(std::iter::once(1).chain(3..)),
        //_ => unimplemented!(),
    }
}

pub struct SeqDigits(Box<dyn Iterator<Item = usize>>, Digits);

pub fn seq_digs(n: usize) -> SeqDigits {
    SeqDigits(seq(n), Digits::new(2, n))
}

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
        //println!("inc {:?}", self.1);
        self.1[0] += 1;
        for i in 0..self.1.len() {
            //println!("inc lsb {:?}", self.1);
            if self.1[i] == self.0 {
                //println!("carry {:?}", self.1);
                self.1[i] = 0;
                if i + 1 < self.1.len() {
                    self.1[i + 1] += 1;
                } else {
                    self.0 += 1;
                }
                //println!("{:?}", self.1);
            }
        }
    }
}

impl AddAssign<usize> for Digits {
    fn add_assign(&mut self, rhs: usize) {
        for _ in 0..rhs {
            self.inc();
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
        const ALPHABET: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";
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
fn test_digits_add() {
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
