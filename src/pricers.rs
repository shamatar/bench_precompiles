pub struct ConstantPricer {
    pub constant: u64
}

pub struct LinearPricer{
    pub constant: u64,
    pub scalar_shift: u64,
    pub scalar_chunk_size: u64,
    pub per_chunk: u64
}

pub enum Pricer {
    Constant(ConstantPricer),
    Linear(LinearPricer)
}

// fn ceil_div(a: u64, b: u64) -> u64 {
//     let mut res = a/b;
//     if a % b != 0 {
//         res += 1;
//     }

//     res
// }

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
                let chunks = floor_div(scalar + inner.scalar_shift, inner.scalar_chunk_size);

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
        per_chunk: 12
    };

    Pricer::Linear(l)
}

pub fn proposed_sha256_pricer() -> Pricer {
    let l = LinearPricer {
        constant: 5,
        scalar_shift: 8,
        scalar_chunk_size: 64,
        per_chunk: 9
    };

    Pricer::Linear(l)
}