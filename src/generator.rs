use super::{runners, input_generators, measurements};

pub const MGAS_PER_SECOND: u128 = 30_000_000;

use rand::{SeedableRng};
use rand_xorshift::XorShiftRng;
use indicatif::{ProgressBar, ProgressStyle};

pub fn generate_sha256_vectors(num_different_vectors: usize, num_tries_per_vector: usize) -> Vec<(u64, Vec<(Vec<u8>, [u8; 32])>, u128, u64)> {    
    let limit = 256;
    let step = 8;

    let mut rng = XorShiftRng::from_seed([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

    let mut data_points = vec![];

    for len in (0..=limit).step_by(step) {
        let mut total = 0u128;
        let mut inputs_and_outputs = vec![];
        for _ in 0..num_different_vectors {
            let (input, output) = input_generators::generate_sha256_vector_for_len(len, &mut rng);

            let input_clone = input.clone();
            let runnable = move || {
                runners::run_sha256(&input_clone)
            };

            let checker = move |r: [u8; 32]| {
                r == output
            };

            let total_time = measurements::measure_with_validity(&runnable, &checker, num_tries_per_vector);
            total += total_time;
            inputs_and_outputs.push((input, output));
        }

        let average_ns = total / (num_different_vectors as u128) / (num_tries_per_vector as u128);
        let gas = average_ns * MGAS_PER_SECOND / 1_000_000_000;

        let gas = gas as u64;

        data_points.push((len as u64, inputs_and_outputs, average_ns, gas));
    }

    data_points
}

pub fn generate_ripemd_vectors(num_different_vectors: usize, num_tries_per_vector: usize) -> Vec<(u64, Vec<(Vec<u8>, [u8; 20])>, u128, u64)> {    
    let limit = 256;
    let step = 8;

    let mut rng = XorShiftRng::from_seed([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

    let mut data_points = vec![];

    for len in (0..=limit).step_by(step) {
        let mut total = 0u128;
        let mut inputs_and_outputs = vec![];
        for _ in 0..num_different_vectors {
            let (input, output) = input_generators::generate_ripemd_vector_for_len(len, &mut rng);

            let input_clone = input.clone();
            let runnable = move || {
                runners::run_ripemd160(&input_clone)
            };

            let checker = move |r: [u8; 20]| {
                r == output
            };

            let total_time = measurements::measure_with_validity(&runnable, &checker, num_tries_per_vector);
            total += total_time;
            inputs_and_outputs.push((input, output));
        }

        let average_ns = total / (num_different_vectors as u128) / (num_tries_per_vector as u128);
        let gas = average_ns * MGAS_PER_SECOND / 1_000_000_000;

        let gas = gas as u64;

        data_points.push((len as u64, inputs_and_outputs, average_ns, gas));
    }

    data_points
}

pub fn generate_blake2f_vectors(num_different_vectors: usize, num_tries_per_vector: usize) -> Vec<(u64, Vec<(Vec<u8>, [u8; 64])>, u128, u64)> {    
    let num_rounds = vec![1, 2, 3, 4, 8, 16, 32, 64, 128];

    let mut rng = XorShiftRng::from_seed([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

    let mut data_points = vec![];

    for rounds in num_rounds.into_iter() {
        let mut total = 0u128;
        let mut inputs_and_outputs = vec![];
        for _ in 0..num_different_vectors {
            let (input, output) = input_generators::generate_blake2f_vector_for_num_rounds(rounds, &mut rng);

            let input_clone = input.clone();
            let runnable = move || {
                runners::run_blake2f(&input_clone)
            };

            let checker = move |r: [u8; 64]| {
                &r[..] == &output[..]
            };

            let total_time = measurements::measure_with_validity(&runnable, &checker, num_tries_per_vector);
            total += total_time;
            inputs_and_outputs.push((input, output));
        }

        let average_ns = total / (num_different_vectors as u128) / (num_tries_per_vector as u128);
        let gas = average_ns * MGAS_PER_SECOND / 1_000_000_000;

        let gas = gas as u64;

        data_points.push((rounds as u64, inputs_and_outputs, average_ns, gas));
    }

    data_points
}

pub fn generate_bn_add_vectors(num_different_vectors: usize, num_tries_per_vector: usize) -> Vec<(u64, Vec<([u8;128], [u8; 64])>, u128, u64)> {    
    let mut rng = XorShiftRng::from_seed([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

    let mut data_points = vec![];

    let pb = make_pb();
    pb.set_length(num_different_vectors as u64);

    let mut total = 0u128;
    let mut inputs_and_outputs = vec![];
    for _ in 0..num_different_vectors {
        let (input, output) = input_generators::generate_bnadd_vector(&mut rng);

        let input_clone = input.clone();
        let runnable = move || {
            runners::run_bn_add(&input_clone)
        };

        let checker = move |r: [u8; 64]| {
            assert!(&r[..] != &[0u8; 64][..]);
            &r[..] == &output[..]
        };

        let total_time = measurements::measure_with_validity(&runnable, &checker, num_tries_per_vector);
        total += total_time;
        inputs_and_outputs.push((input, output));
        pb.inc(1);
    }

    let average_ns = total / (num_different_vectors as u128) / (num_tries_per_vector as u128);
    let gas = average_ns * MGAS_PER_SECOND / 1_000_000_000;

    let gas = gas as u64;

    data_points.push((0u64, inputs_and_outputs, average_ns, gas));

    data_points
}


pub fn generate_bn_mul_vectors(num_different_vectors: usize, num_tries_per_vector: usize) -> Vec<(u64, Vec<([u8; 96], [u8; 64])>, u128, u64)> {    
    let mut rng = XorShiftRng::from_seed([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

    let mut data_points = vec![];

    let pb = make_pb();
    pb.set_length(num_different_vectors as u64);

    let mut total = 0u128;
    let mut inputs_and_outputs = vec![];
    for _ in 0..num_different_vectors {
        let (input, output) = input_generators::generate_bnmul_vector(&mut rng);

        let input_clone = input.clone();
        let runnable = move || {
            runners::run_bn_mul(&input_clone)
        };

        let checker = move |r: [u8; 64]| {
            assert!(&r[..] != &[0u8; 64][..]);
            &r[..] == &output[..]
        };

        let total_time = measurements::measure_with_validity(&runnable, &checker, num_tries_per_vector);
        total += total_time;
        inputs_and_outputs.push((input, output));
        pb.inc(1);
    }

    let average_ns = total / (num_different_vectors as u128) / (num_tries_per_vector as u128);
    let gas = average_ns * MGAS_PER_SECOND / 1_000_000_000;

    let gas = gas as u64;

    data_points.push((0u64, inputs_and_outputs, average_ns, gas));

    data_points
}

pub fn generate_bnpair_vectors(num_different_vectors: usize, num_tries_per_vector: usize) -> Vec<(u64, Vec<(Vec<u8>, [u8; 32])>, u128, u64)> {    
    let num_pairs = vec![1, 2, 4, 8];

    let mut rng = XorShiftRng::from_seed([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

    let mut data_points = vec![];

    let pb = make_pb();
    pb.set_length((num_pairs.len() * num_different_vectors) as u64);

    for pairs in num_pairs.into_iter() {
        let mut total = 0u128;
        let mut inputs_and_outputs = vec![];
        for _ in 0..num_different_vectors {
            let (input, output) = input_generators::generate_bnpair_vector(pairs, &mut rng);

            let input_clone = input.clone();
            let runnable = move || {
                runners::run_bn_pair(&input_clone)
            };

            let checker = move |r: [u8; 32]| {
                r == output
            };

            let total_time = measurements::measure_with_validity(&runnable, &checker, num_tries_per_vector);
            total += total_time;
            inputs_and_outputs.push((input, output));
            pb.inc(1);
        }

        let average_ns = total / (num_different_vectors as u128) / (num_tries_per_vector as u128);
        let gas = average_ns * MGAS_PER_SECOND / 1_000_000_000;

        let gas = gas as u64;

        data_points.push((pairs as u64, inputs_and_outputs, average_ns, gas));
    }

    data_points
}

pub fn perform_measurements<
    T, 
    F: Fn() -> Vec<(u64, Vec<T>, u128, u64)>, 
    C: Fn(T) -> (Vec<u8>, Vec<u8>),
    A: Fn(u64) -> String
>(
    should_write: bool,
    current_pricer: crate::pricers::Pricer,
    proposed_pricer: crate::pricers::Pricer,
    runner: F,
    transformer: C,
    base_path: &str,
    ann: A
) {
    let data = runner();
    for (scalar_param, ins_and_outs, _, gas) in data.into_iter() {
        let data_as_vector: Vec<_> = ins_and_outs.into_iter().map(|el| transformer(el)).collect();
        let current_gas = current_pricer.price(scalar_param);
        let proposed_gas = proposed_pricer.price(scalar_param);
        if should_write {
            for (p, g) in vec!["current", "proposed"].into_iter().zip(vec![current_gas, proposed_gas].into_iter()) {
                let file = std::fs::File::create(&format!("{}/{}/input_param_scalar_{}_gas_{}.csv", base_path, p, scalar_param, g)).unwrap();
                let mut writer = csv::Writer::from_writer(file);
                for (input, output) in data_as_vector.clone().into_iter() {
                    writer.write_record(&[
                        hex::encode(&input),
                        hex::encode(&output)
                    ]).unwrap();
                }
            }
        }

        let annotation = ann(scalar_param);

        println!("{}", annotation);
        print_gases(gas, current_gas, proposed_gas);
    }
}

fn make_colored_bool(input: bool) -> colored::ColoredString {
    use colored::*;

    if input {
        format!("{}", input).green()
    } else {
        format!("{}", input).red()
    }
}

fn print_gases(real: u64, current_schedule: u64, proposed_schedule: u64) {
    use colored::*;

    println!("Fits into pre-2666 schedule: {}, runtime: {}, schedule: {}", 
        make_colored_bool(real <= current_schedule), 
        format!("{}", real).yellow(),
        format!("{}", current_schedule).yellow()
    );
    println!("Fits into post-2666 schedule: {}, runtime: {}, schedule: {}", 
        make_colored_bool(real <= proposed_schedule), 
        format!("{}", real).yellow(),
        format!("{}", proposed_schedule).yellow()
    );
}

fn make_pb() -> ProgressBar {
    let pb = ProgressBar::new(1);

    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}|{eta_precise}] {bar:50} {pos:>7}/{len:7} {msg}")
        .progress_chars("##-"));

    pb
}

#[cfg(test)]
mod test {
    use super::*;

    fn do_sha256(write: bool) {
        let base_path = "./vectors/sha256";

        let data_fn = || {
            generate_sha256_vectors(10, 10000)
        };

        let transformer_fn = |a: (Vec<u8>, [u8; 32])| {
            let (a, b) = a;

            (a, b.to_vec())
        };

        let ann_fn = |len: u64| {
            format!("For length {}:", len)
        };
        
        perform_measurements(
            write,
            crate::pricers::current_sha256_pricer(),
            crate::pricers::proposed_sha256_pricer(),
            data_fn,
            transformer_fn,
            base_path,
            ann_fn
        );
    }

    fn do_ripemd(write: bool) {
        let base_path = "./vectors/ripemd";

        let data_fn = || {
            generate_ripemd_vectors(10, 10000)
        };

        let transformer_fn = |a: (Vec<u8>, [u8; 20])| {
            let (a, b) = a;

            (a, b.to_vec())
        };

        let ann_fn = |len: u64| {
            format!("For length {}:", len)
        };
        
        perform_measurements(
            write,
            crate::pricers::current_ripemd_pricer(),
            crate::pricers::proposed_ripemd_pricer(),
            data_fn,
            transformer_fn,
            base_path,
            ann_fn
        );
    }

    fn do_blake2f(write: bool) {
        let base_path = "./vectors/blake2f";

        let data_fn = || {
            generate_blake2f_vectors(10, 10000)
        };

        let transformer_fn = |a: (Vec<u8>, [u8; 64])| {
            let (a, b) = a;

            (a, b.to_vec())
        };

        let ann_fn = |rounds: u64| {
            format!("For {} rounds:", rounds)
        };
        
        perform_measurements(
            write,
            crate::pricers::blake2f_pricer(),
            crate::pricers::blake2f_pricer(),
            data_fn,
            transformer_fn,
            base_path,
            ann_fn
        );
    }

    fn do_bnadd(write: bool) {
        let base_path = "./vectors/bnadd";

        let data_fn = || {
            generate_bn_add_vectors(10, 10000)
        };

        let transformer_fn = |a: ([u8; 128], [u8; 64])| {
            let (a, b) = a;

            (a.to_vec(), b.to_vec())
        };

        let ann_fn = |_: u64| {
            String::from("")
        };
        
        perform_measurements(
            write,
            crate::pricers::current_bnadd_pricer(),
            crate::pricers::proposed_bnadd_pricer(),
            data_fn,
            transformer_fn,
            base_path,
            ann_fn
        );
    }

    fn do_bnmul(write: bool) {
        let base_path = "./vectors/bnmul";

        let data_fn = || {
            generate_bn_mul_vectors(10, 10000)
        };

        let transformer_fn = |a: ([u8; 96], [u8; 64])| {
            let (a, b) = a;

            (a.to_vec(), b.to_vec())
        };

        let ann_fn = |_: u64| {
            String::from("")
        };
        
        perform_measurements(
            write,
            crate::pricers::current_bnmul_pricer(),
            crate::pricers::proposed_bnmul_pricer(),
            data_fn,
            transformer_fn,
            base_path,
            ann_fn
        );
    }


    fn do_bnpair(write: bool) {
        let base_path = "./vectors/bnpair";

        let data_fn = || {
            generate_bnpair_vectors(10, 1000)
        };

        let transformer_fn = |a: (Vec<u8>, [u8; 32])| {
            let (a, b) = a;

            (a, b.to_vec())
        };

        let ann_fn = |num_pairs: u64| {
            format!("For {} pairs", num_pairs)
        };
        
        perform_measurements(
            write,
            crate::pricers::bnpair_pricer(),
            crate::pricers::bnpair_pricer(),
            data_fn,
            transformer_fn,
            base_path,
            ann_fn
        );
    }

    #[test]
    fn generate_sha256() {
        do_sha256(true);
    }

    #[test]
    fn try_sha256() {
        do_sha256(false);
    }

    #[test]
    fn generate_ripemd() {
        do_ripemd(true);
    }

    #[test]
    fn try_ripemd() {
        do_ripemd(false);
    }

    #[test]
    fn generate_blake2f() {
        do_blake2f(true);
    }

    #[test]
    fn try_blake2f() {
        do_blake2f(false);
    }

    #[test]
    fn generate_bnadd() {
        do_bnadd(true);
    }

    #[test]
    fn try_bnadd() {
        do_bnadd(false);
    }

    #[test]
    fn generate_bnmul() {
        do_bnmul(true);
    }

    #[test]
    fn try_bnmul() {
        do_bnmul(false);
    }

    #[test]
    fn generate_bnpair() {
        do_bnpair(true);
    }

    #[test]
    fn try_bnpair() {
        do_bnpair(false);
    }
}