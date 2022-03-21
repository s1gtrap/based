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
                    for i in 0..(b - 1 + b) {
                        if i < b - 1 {
                            yield b;
                        } else {
                            yield 1;
                        }
                    }
                }
                (3, b) => {
                    for i in 0..((b - 1) * (b - 1 + b)) {
                        if i % (b - 1 + b) < b - 1 {
                            yield b;
                        } else {
                            yield 1;
                        }
                    }
                    for _ in 0..(b * b) {
                        yield 1;
                    }
                }
                (n @ 4, b) => {
                    let np = n - 1;
                    let npp = n - 2;
                    let bnp = usize::pow(b, np as _);
                    let bnpp = usize::pow(b, npp as _);
                    let skip = b - 1;
                    let pass = b;
                    let fst_len = skip + pass;
                    let snd_len = skip * fst_len + bnpp;
                    for i in 0..(skip * snd_len) {
                        if i % snd_len % fst_len < skip && i % snd_len < skip * fst_len {
                            yield b;
                        } else {
                            yield 1;
                        }
                    }
                    for _ in 0..bnp {
                        yield 1;
                    }
                }
                (5, b) => {
                    let np = n - 1;
                    let npp = n - 2;
                    let nppp = n - 3;
                    let bnp = usize::pow(b, np as _);
                    let bnpp = usize::pow(b, npp as _);
                    let bnppp = usize::pow(b, nppp as _);
                    let skip = b - 1;
                    let pass = b;
                    let fst_len = skip + pass;
                    let snd_len = skip * fst_len + bnppp;
                    let trd_len = skip * snd_len + bnpp;
                    for i in 0..(skip * trd_len) {
                        if i % trd_len % snd_len % fst_len < skip
                            && i % trd_len % snd_len < skip * fst_len
                            && i % trd_len < skip * snd_len
                        {
                            yield b;
                        } else {
                            yield 1;
                        }
                    }
                    for _ in 0..bnp {
                        yield 1;
                    }
                }
                (6, b) => {
                    for _ in 0..(b - 1) {
                        for _ in 0..(b - 1) {
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
                        for _ in 0..(b * b * b * b) {
                            yield 1;
                        }
                    }
                    for _ in 0..(b * b * b * b * b) {
                        yield 1;
                    }
                }
                (7, b) => {
                    for _ in 0..(b - 1) {
                        for _ in 0..(b - 1) {
                            for _ in 0..(b - 1) {
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
                            for _ in 0..(b * b * b * b) {
                                yield 1;
                            }
                        }
                        for _ in 0..(b * b * b * b * b) {
                            yield 1;
                        }
                    }
                    for _ in 0..(b * b * b * b * b * b) {
                        yield 1;
                    }
                }
                (8, b) => {
                    for _ in 0..(b - 1) {
                        for _ in 0..(b - 1) {
                            for _ in 0..(b - 1) {
                                for _ in 0..(b - 1) {
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
                                for _ in 0..(b * b * b * b) {
                                    yield 1;
                                }
                            }
                            for _ in 0..(b * b * b * b * b) {
                                yield 1;
                            }
                        }
                        for _ in 0..(b * b * b * b * b * b) {
                            yield 1;
                        }
                    }
                    for _ in 0..(b * b * b * b * b * b) {
                        yield 1;
                    }
                }
                (9, b) => {
                    for _ in 0..(b - 1) {
                        for _ in 0..(b - 1) {
                            for _ in 0..(b - 1) {
                                for _ in 0..(b - 1) {
                                    for _ in 0..(b - 1) {
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
                                    for _ in 0..(b * b * b * b) {
                                        yield 1;
                                    }
                                }
                                for _ in 0..(b * b * b * b * b) {
                                    yield 1;
                                }
                            }
                            for _ in 0..(b * b * b * b * b * b) {
                                yield 1;
                            }
                        }
                        for _ in 0..(b * b * b * b * b * b * b) {
                            yield 1;
                        }
                    }
                    for _ in 0..(b * b * b * b * b * b * b) {
                        yield 1;
                    }
                }
                w => unimplemented!("{:?}", w),
            }
        }
    }
}
