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
                    let n_prev = n - 1;
                    let n_prev_prev = n - 2;
                    let bn_prev = usize::pow(b, n_prev as _);
                    let bn_prev_prev = usize::pow(b, n_prev_prev as _);
                    let skip = b - 1;
                    let pass = b;
                    let fst_iter_len = skip + pass;
                    let snd_iter_len = skip * fst_iter_len + bn_prev_prev;
                    for i in 0..(skip * snd_iter_len) {
                        if i % snd_iter_len % fst_iter_len < skip
                            && i % snd_iter_len < skip * fst_iter_len
                        {
                            yield b;
                        } else {
                            yield 1;
                        }
                    }
                    for _ in 0..bn_prev {
                        yield 1;
                    }
                }
                (5, b) => {
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
