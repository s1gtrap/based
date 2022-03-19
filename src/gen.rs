use std::ops::Generator;

/*macro_rules! skip {
    ( $n:expr ) => {
        yield $n + 1;
    };
}*/

pub fn gen(n: usize) -> impl Generator<Yield = usize, Return = ()> {
    move || {
        for b in 2.. {
            match (n, b) {
                (_, 2) => {
                    for _ in 0..(2usize.pow(n as _) - 1) {
                        yield 1;
                    }
                }
                (2, b) => {
                    for _ in 0..(b - 1) {
                        yield b;
                    }
                    for _ in 0..b {
                        yield 1;
                    }
                }
                (3, b) => {
                    for _ in 0..(b - 1) {
                        for _ in 0..(b - 1) {
                            yield b;
                        }
                        for _ in 0..b {
                            yield 1;
                        }
                    }
                    for _ in 0..(b * b) {
                        yield 1;
                    }
                }
                (4, b) => {
                    for _ in 0..(b - 1) {
                        for _ in 0..(b - 1) {
                            for _ in 0..(b - 1) {
                                yield b;
                            }
                            for _ in 0..b {
                                yield 1;
                            }
                        }
                        for _ in 0..(b * b) {
                            yield 1;
                        }
                    }
                    for _ in 0..(b * b * b) {
                        yield 1;
                    }
                }
                w => unimplemented!("{:?}", w),
            }
        }
    }
}
