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
                    let np = n - 1;
                    let bnp = usize::pow(b, np as _);
                    let skip = b - 1;
                    let pass = b;
                    let fst_len = skip + pass;
                    for i in 0..(skip * fst_len) {
                        if i % fst_len < skip {
                            yield b;
                        } else {
                            yield 1;
                        }
                    }
                    for _ in 0..bnp {
                        yield 1;
                    }
                }
                (n @ 4, b) => {
                    let skip = b - 1;
                    let pass = b;
                    let mut len = Vec::with_capacity(n - 2);
                    len.insert(0, skip + pass);
                    for i in 1..n - 2 {
                        len.insert(i, skip * len[i - 1] + usize::pow(b, (n - i - 1) as _));
                    }
                    for i in 0..(skip * len[len.len() - 1]) {
                        let mut j = 1;
                        if i % len[1] % len[0] < skip && i % len[1] < skip * len[0] {
                            yield b;
                        } else {
                            yield 1;
                        }
                    }
                    for _ in 0..usize::pow(b, (n - 1) as _) {
                        yield 1;
                    }
                }
                (n @ 5, b) => {
                    let skip = b - 1;
                    let pass = b;
                    let mut len = Vec::with_capacity(n - 2);
                    len.insert(0, skip + pass);
                    for i in 1..n - 2 {
                        len.insert(i, skip * len[i - 1] + usize::pow(b, (n + i - 4) as _));
                    }
                    for i in 0..(skip * len[len.len() - 1]) {
                        let mut j = 1;
                        if i % len[2] % len[1] % len[0] < skip
                            && i % len[2] % len[1] < skip * len[0]
                            && i % len[2] < skip * len[1]
                        {
                            yield b;
                        } else {
                            yield 1;
                        }
                    }
                    for _ in 0..usize::pow(b, (n - 1) as _) {
                        yield 1;
                    }
                }
                (6, b) => {
                    let np = n - 1;
                    let npp = n - 2;
                    let nppp = n - 3;
                    let npppp = n - 4;
                    let bnp = usize::pow(b, np as _);
                    let bnpp = usize::pow(b, npp as _);
                    let bnppp = usize::pow(b, nppp as _);
                    let bnpppp = usize::pow(b, npppp as _);
                    let skip = b - 1;
                    let pass = b;
                    let fst_len = skip + pass;
                    let snd_len = skip * fst_len + bnpppp;
                    let trd_len = skip * snd_len + bnppp;
                    let frt_len = skip * trd_len + bnpp;
                    for i in 0..(skip * frt_len) {
                        if i % frt_len % trd_len % snd_len % fst_len < skip
                            && i % frt_len % trd_len % snd_len < skip * fst_len
                            && i % frt_len % trd_len < skip * snd_len
                            && i % frt_len < skip * trd_len
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
