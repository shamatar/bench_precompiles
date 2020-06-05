pub struct ConstantPricer {
    pub constant: u64
}

pub struct LinearPricer{
    pub constant: u64,
    pub scalar_shift: u64,
    pub scalar_chunk_size: u64,
    pub per_chunk: u64,
    pub use_ceil_div: bool,
}

pub enum Pricer {
    Constant(ConstantPricer),
    Linear(LinearPricer)
}

fn ceil_div(a: u64, b: u64) -> u64 {
    let mut res = a/b;
    if a % b != 0 {
        res += 1;
    }

    res
}

fn floor_div(a: u64, b: u64) -> u64 {
    a/b
}

impl Pricer {
    pub fn price(&self, scalar: u64) -> u64 {
        match self {
            Pricer::Constant(inner) => {
                inner.constant
            },
            Pricer::Linear(inner) => {
                let chunks = if inner.use_ceil_div {
                    ceil_div(scalar + inner.scalar_shift, inner.scalar_chunk_size)
                } else {
                    floor_div(scalar + inner.scalar_shift, inner.scalar_chunk_size)
                };

                inner.constant + chunks*inner.per_chunk
            }
        }
    }
}

pub fn current_sha256_pricer() -> Pricer {
    let l = LinearPricer {
        constant: 60,
        scalar_shift: 0,
        scalar_chunk_size: 32,
        per_chunk: 12,
        use_ceil_div: true,
    };

    Pricer::Linear(l)
}

pub fn proposed_sha256_pricer() -> Pricer {
    let l = LinearPricer {
        constant: 14,
        scalar_shift: 8,
        scalar_chunk_size: 64,
        per_chunk: 9,
        use_ceil_div: false
    };

    Pricer::Linear(l)
}


pub fn current_ripemd_pricer() -> Pricer {
    let l = LinearPricer {
        constant: 600,
        scalar_shift: 0,
        scalar_chunk_size: 32,
        per_chunk: 120,
        use_ceil_div: true,
    };

    Pricer::Linear(l)
}

pub fn proposed_ripemd_pricer() -> Pricer {
    let l = LinearPricer {
        constant: 18,
        scalar_shift: 8,
        scalar_chunk_size: 64,
        per_chunk: 12,
        use_ceil_div: false
    };

    Pricer::Linear(l)
}

pub fn current_bnadd_pricer() -> Pricer {
    let l = ConstantPricer {
        constant: 150
    };

    Pricer::Constant(l)
}

pub fn proposed_bnadd_pricer() -> Pricer {
    let l = ConstantPricer {
        constant: 350
    };

    Pricer::Constant(l)
}

pub fn current_bnmul_pricer() -> Pricer {
    let l = ConstantPricer {
        constant: 6000
    };

    Pricer::Constant(l)
}

pub fn proposed_bnmul_pricer() -> Pricer {
    let l = ConstantPricer {
        constant: 6300
    };

    Pricer::Constant(l)
}

pub fn bnpair_pricer() -> Pricer {
    let l = LinearPricer {
        constant: 45000,
        scalar_shift: 0,
        scalar_chunk_size: 1,
        per_chunk: 34000,
        use_ceil_div: false
    };

    Pricer::Linear(l)
}

pub fn blake2f_pricer() -> Pricer {
    let l = LinearPricer {
        constant: 0,
        scalar_shift: 0,
        scalar_chunk_size: 1,
        per_chunk: 1,
        use_ceil_div: false
    };

    Pricer::Linear(l)
}