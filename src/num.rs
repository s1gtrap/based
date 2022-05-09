use std::ops::{Add, AddAssign};
use std::{fmt, iter};

#[derive(Clone, PartialEq)]
pub struct Int {
    pub(crate) base: usize,
    pub(crate) digs: Vec<usize>,
}

impl Int {
    pub fn new(len: usize) -> Self {
        Self::new_with_base(len, 2)
    }

    pub fn new_with_base(len: usize, base: usize) -> Self {
        Int {
            base,
            digs: vec![0; len],
        }
    }

    pub fn digits(&self) -> &[usize] {
        &self.digs[..]
    }
}

impl fmt::Debug for Int {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const ALPHABET: &[u8; 62] =
            b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

        match self.base {
            2 => write!(f, "0b"),
            8 => write!(f, "0o"),
            16 => write!(f, "0x"),
            r => write!(f, "{}r", r),
        }?;
        for c in self.digs.iter().rev() {
            write!(f, "{}", ALPHABET[*c] as char)?;
        }
        Ok(())
    }
}

impl From<Int> for usize {
    fn from(other: Int) -> usize {
        let mut r = 0;
        for d in other.digs.iter().rev() {
            r *= other.base;
            r += d;
        }
        r
    }
}

impl<'a> From<&'a Int> for usize {
    fn from(other: &'a Int) -> usize {
        let mut r = 0;
        for d in other.digs.iter().rev() {
            r *= other.base;
            r += d;
        }
        r
    }
}

impl From<(u8, usize)> for Int {
    fn from((base, mut num): (u8, usize)) -> Self {
        let base = base as _;
        let mut digs = vec![];
        while num > 0 {
            digs.push(num % base);
            num /= base;
        }
        Int { base, digs: digs }
    }
}

impl From<(u8, usize, usize)> for Int {
    fn from((base, len, mut num): (u8, usize, usize)) -> Self {
        let base = base as _;
        let mut digs = vec![];
        while num > 0 {
            digs.push(num % base);
            num /= base;
        }
        Int {
            base,
            digs: digs.into_iter().chain(iter::repeat(0)).take(len).collect(),
        }
    }
}

impl<I> From<(usize, I)> for Int
where
    I: IntoIterator<Item = usize>,
{
    fn from((base, digs): (usize, I)) -> Self {
        Int {
            base,
            digs: digs.into_iter().collect(),
        }
    }
}

impl<I> From<(usize, usize, I)> for Int
where
    I: IntoIterator<Item = usize>,
{
    fn from((base, len, digs): (usize, usize, I)) -> Self {
        Int {
            base,
            digs: digs.into_iter().chain(iter::repeat(0)).take(len).collect(),
        }
    }
}

impl AddAssign<usize> for Int {
    fn add_assign(&mut self, rhs: usize) {
        let usz = usize::from(&*self);
        *self = Self::from((
            (self.base + (usz + rhs) / usize::pow(self.base, self.digs.len() as _)) as u8,
            self.digs.len(),
            (usz + rhs) % usize::pow(self.base, self.digs.len() as _),
        ));
        self.base += (usz + rhs) / usize::pow(self.base, self.digs.len() as _);
    }
}

impl AddAssign for Int {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::from((self.base as u8, usize::from(&*self) + usize::from(rhs)));
        /*assert_eq!(self.base, rhs.base);

        for (i, (a, b)) in self
            .digs
            .clone()
            .iter()
            .copied()
            .chain(iter::repeat(0))
            .zip(rhs.digs.iter().copied().chain(iter::repeat(0)))
            .take(usize::max(self.digs.len(), rhs.digs.len()))
            .enumerate()
        {
            self.digs[i] = (a + b) % self.base;
        }*/
    }
}

impl Add<usize> for Int {
    type Output = Self;
    fn add(mut self, rhs: usize) -> Self {
        self += rhs;
        self
    }
}

impl Add for Int {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

#[test]
fn test_int_from_iter() {
    assert_eq!(
        Int::from((2, [0, 1, 0, 1])),
        Int {
            base: 2,
            digs: vec![0, 1, 0, 1],
        },
    );
    assert_eq!(
        Int::from((8, vec![4, 2])),
        Int {
            base: 8,
            digs: vec![4, 2],
        },
    );
}

#[test]
fn test_int_from_usize() {
    assert_eq!(
        Int::from((2, 0b1010)),
        Int {
            base: 2,
            digs: vec![0, 1, 0, 1],
        },
    );
    assert_eq!(
        Int::from((8, 0o755)),
        Int {
            base: 8,
            digs: vec![5, 5, 7],
        },
    );
    assert_eq!(
        Int::from((10, u32::MAX as usize)),
        Int {
            base: 10,
            digs: vec![5, 9, 2, 7, 6, 9, 4, 9, 2, 4],
        },
    );
}

#[test]
fn test_int_into_usize() {
    assert_eq!(
        <Int as Into<usize>>::into(Int {
            base: 2,
            digs: vec![0, 1, 0, 1],
        }),
        0b1010,
    );
    assert_eq!(
        <Int as Into<usize>>::into(Int {
            base: 8,
            digs: vec![5, 5, 7],
        }),
        0o755,
    );
    assert_eq!(
        <Int as Into<usize>>::into(Int {
            base: 10,
            digs: vec![5, 9, 2, 7, 6, 9, 4, 9, 2, 4],
        }),
        u32::MAX as usize,
    );
}

#[test]
fn test_int_add_int() {
    assert_eq!(
        Int::from((2, 0b1010)) + Int::from((2, 0b0101)),
        Int::from((2, 0b1111)),
    );
}
