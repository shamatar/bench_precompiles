use super::{runners, input_generators, measurements};

pub const MGAS_PER_SECOND: u128 = 30_000_000;

use rand::{SeedableRng};
use rand_xorshift::XorShiftRng;

pub fn generate_sha256_vectors(num_different_vectors: usize, num_tries_per_vector: usize) -> Vec<( Vec<(Vec<u8>, [u8; 32])>, u128, u64)> {    
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
                runners::run_sha256(&input_clone);
                Ok(())
            };

            let total_time = measurements::measure(&runnable, num_tries_per_vector);
            total += total_time;
            inputs_and_outputs.push((input, output));
        }

        let average_ns = total / (num_different_vectors as u128) / (num_tries_per_vector as u128);
        let gas = average_ns * MGAS_PER_SECOND / 1_000_000_000;

        let gas = gas as u64;

        data_points.push((inputs_and_outputs, average_ns, gas));
    }

    data_points
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_for_sha256_current_pricing() {
        let base_path = "./vectors/sha256/current";
        let data = generate_sha256_vectors(10, 10000);
        for (ins_and_outs, _, _) in data.into_iter() {
            let input_len = ins_and_outs[0].0.len();
            let pricer = crate::pricers::current_sha256_pricer();
            let gas = pricer.price(input_len as u64);
            let file = std::fs::File::create(&format!("{}/input_len_{}_gas_{}", base_path, input_len, gas)).unwrap();
            let mut writer = csv::Writer::from_writer(file);
            for (input, output) in ins_and_outs.into_iter() {
                writer.write_record(&[
                    hex::encode(&input),
                    hex::encode(&output)
                ]).unwrap();
            }
        }
    }

    #[test]
    fn generate_for_sha256_proposed_pricing() {
        let base_path = "./vectors/sha256/proposed";
        let data = generate_sha256_vectors(10, 10000);
        for (ins_and_outs, _, _) in data.into_iter() {
            let input_len = ins_and_outs[0].0.len();
            let pricer = crate::pricers::proposed_sha256_pricer();
            let gas = pricer.price(input_len as u64);
            let file = std::fs::File::create(&format!("{}/input_len_{}_gas_{}", base_path, input_len, gas)).unwrap();
            let mut writer = csv::Writer::from_writer(file);
            for (input, output) in ins_and_outs.into_iter() {
                writer.write_record(&[
                    hex::encode(&input),
                    hex::encode(&output)
                ]).unwrap();
            }
        }
    }


    #[test]
    fn try_for_sha256() {
        let data = generate_sha256_vectors(10, 10000);
        for (ins_and_outs, _, gas) in data.into_iter() {
            let input_len = ins_and_outs[0].0.len();
            let pricer = crate::pricers::current_sha256_pricer();
            let gas_before_2666 = pricer.price(input_len as u64);
            let pricer = crate::pricers::proposed_sha256_pricer();
            let gas_after_2666 = pricer.price(input_len as u64);
            println!("For length {}:", input_len);
            println!("Fits into pre-2666 schedule: {}, runtime: {}, schedule: {}", gas <= gas_before_2666, gas, gas_before_2666);
            println!("Fits into post-2666 schedule: {}, runtime: {}, schedule: {}", gas <= gas_after_2666, gas, gas_after_2666);
        }
    }
}