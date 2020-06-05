use rand::{Rng};

use super::runners;
use super::helpers;

pub fn generate_random_bytes_for_length<R: Rng>(len: usize, rng: &mut R) -> Vec<u8> {
    let mut input = vec![0u8; len];
    rng.fill_bytes(&mut input);

    input
}

pub fn generate_sha256_vector_for_len<R: Rng>(input_len: usize, rng: &mut R) -> (Vec<u8>, [u8; 32]) {
    let input = generate_random_bytes_for_length(input_len, rng);
    let output = runners::run_sha256(&input);

    (input, output)
}

pub fn generate_ripemd_vector_for_len<R: Rng>(input_len: usize, rng: &mut R) -> (Vec<u8>, [u8; 20]) {
    let input = generate_random_bytes_for_length(input_len, rng);
    let output = runners::run_ripemd160(&input);

    (input, output)
}

pub fn generate_blake2f_vector_for_num_rounds<R: Rng>(rounds: usize, rng: &mut R) -> (Vec<u8>, [u8; 64]) {
    use byteorder::{BigEndian};
    use byteorder::{WriteBytesExt};

    const BLAKE2_F_ARG_LEN: usize = 213;

    let mut input = vec![0u8; BLAKE2_F_ARG_LEN];
    (&mut input[0..4]).write_u32::<BigEndian>(rounds as u32).expect("must write number of rounds");
        
    rng.fill_bytes(&mut input[4..]);
    let last_byte = input[BLAKE2_F_ARG_LEN-1];
    input[BLAKE2_F_ARG_LEN-1] = last_byte & 1u8;

    let output = runners::run_blake2f(&input);

    (input, output)
}

pub fn generate_random_g1_points<R: Rng>(rng: &mut R) -> bn::AffineG1 {
    let mut scalar_buffer = vec![0u8; 32];
    let mut base_point_x = vec![0u8; 32];
    base_point_x[31] = 1;
    let mut base_point_y = vec![0u8; 32];
    base_point_y[31] = 2;
    let mut base_point_encoding = vec![];
    base_point_encoding.extend(base_point_x);
    base_point_encoding.extend(base_point_y);

    let mut input = vec![0u8; 128];

    let base_point = helpers::read_point(&mut base_point_encoding).unwrap();

    rng.fill_bytes(&mut scalar_buffer);
    scalar_buffer[0] = 0;

    let fr = helpers::read_fr(&mut scalar_buffer).unwrap();

    let p1 = bn::AffineG1::from_jacobian(base_point * fr).unwrap();
    p1.x().to_big_endian(&mut input[0..32]).expect("Cannot fail since 0..32 is 32-byte length");
    p1.y().to_big_endian(&mut input[32..64]).expect("Cannot fail since 32..64 is 32-byte length");

    p1
}

pub fn worst_case_scalar_for_double_and_add() -> [u8; 32] {
    [0xff; 32]
}

pub fn generate_bnadd_vector<R: Rng>(rng: &mut R) -> ([u8; 128], [u8; 64]) {
    use std::io::Write;

    let mut input = [0u8; 128];

    let p1 = generate_random_g1_points(rng);
    let p2 = generate_random_g1_points(rng);

    let p1_encoding = helpers::encode_g1_point(p1);
    let p2_encoding = helpers::encode_g1_point(p2);

    (&mut input[0..64]).write(&p1_encoding).unwrap();
    (&mut input[64..128]).write(&p2_encoding).unwrap();

    let output = runners::run_bn_add(&input);

    assert!(&output[..] != &[0u8; 64][..]);

    (input, output)
}

pub fn generate_bnmul_vector<R: Rng>(rng: &mut R) -> ([u8; 96], [u8; 64]) {
    use std::io::Write;

    let mut input = [0u8; 96];

    let p1 = generate_random_g1_points(rng);

    let p1_encoding = helpers::encode_g1_point(p1);

    (&mut input[0..64]).write(&p1_encoding).unwrap();
    (&mut input[64..96]).write(&worst_case_scalar_for_double_and_add()).unwrap();

    let output = runners::run_bn_mul(&input);

    assert!(&output[..] != &[0u8; 64][..]);

    (input, output)
}

pub fn generate_bnpair_vector<R: Rng>(num_pairs: usize, rng: &mut R) -> (Vec<u8>, [u8; 32]) {
    use bn::{Group, AffineG1, AffineG2};
    
    assert!(num_pairs > 0);

    let mut input = vec![0u8; num_pairs * (64 + 128)];

    let mut scalar_buffer = vec![0u8; 32];

    use std::ops::Mul;

    let mut offset = 0;
    for _ in 0..num_pairs {
        rng.fill_bytes(&mut scalar_buffer);
        scalar_buffer[0] = 0;

        let fr = helpers::read_fr(&mut scalar_buffer).unwrap();
        
        let p1 = bn::G1::one().mul(fr);
        let p1 = AffineG1::from_jacobian(p1).unwrap();
        p1.x().to_big_endian(&mut input[offset..(offset+32)]).expect("Cannot fail since 0..32 is 32-byte length");
        offset += 32;
        p1.y().to_big_endian(&mut input[offset..(offset+32)]).expect("Cannot fail since 32..64 is 32-byte length");
        offset += 32;

        rng.fill_bytes(&mut scalar_buffer);
        scalar_buffer[0] = 0;

        let fr = helpers::read_fr(&mut scalar_buffer).unwrap();

        let p2 = bn::G2::one().mul(fr);
        let p2 = AffineG2::from_jacobian(p2).unwrap();
        p2.x().imaginary().to_big_endian(&mut input[offset..(offset+32)]).expect("Cannot fail since 0..32 is 32-byte length");
        offset += 32;
        p2.x().real().to_big_endian(&mut input[offset..(offset+32)]).expect("Cannot fail since 0..32 is 32-byte length");
        offset += 32;

        p2.y().imaginary().to_big_endian(&mut input[offset..(offset+32)]).expect("Cannot fail since 0..32 is 32-byte length");
        offset += 32;
        p2.y().real().to_big_endian(&mut input[offset..(offset+32)]).expect("Cannot fail since 0..32 is 32-byte length");
        offset += 32;
    }

    let output = runners::run_bn_pair(&input);

    (input, output)

}