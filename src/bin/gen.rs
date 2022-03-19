#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

fn main() {
    let n = std::env::args().nth(1).unwrap().parse().unwrap();
    let mut gen = based::gen::gen(n);
    let mut digs = based::Digits::new(2, n);
    for _ in 0.. {
        match Pin::new(&mut gen).resume(()) {
            GeneratorState::Yielded(i) => {
                println!("{} + {:?}", digs, i);
                digs += i;
            }
            _ => unreachable!(),
        }
    }
}
