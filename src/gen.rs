use std::ops::Generator;

/*macro_rules! skip {
    ( $n:expr ) => {
        yield $n + 1;
    };
}*/

pub fn gen(n: usize) -> impl Generator<Yield = usize, Return = ()> {
    gen_with_base(n, 2)
}

pub fn gen_with_base(n: usize, bj: usize) -> impl Generator<Yield = usize, Return = ()> {
    move || {
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
    }
}
