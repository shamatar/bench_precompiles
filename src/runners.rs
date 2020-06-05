use super::helpers;

pub fn run_sha256(input: &[u8]) -> [u8; 32] {
    use parity_crypto::digest;
    use std::io::Write;

    let d = digest::sha256(&input);
    let mut output = [0u8; 32];
    (&mut output[..]).write(&d).unwrap();

    output
}

pub fn run_ripemd160(input: &[u8]) -> [u8; 20] {
    use parity_crypto::digest;
    use std::io::Write;

    let d = digest::ripemd160(&input);
    let mut output = [0u8; 20];
    (&mut output[..]).write(&d).unwrap();

    output
}

pub fn run_blake2f(input: &[u8]) -> [u8; 64] {
    use std::io::{Cursor, Write};
    use byteorder::{BigEndian, LittleEndian};
    use byteorder::{ReadBytesExt};
    use eip_152::compress;

    const BLAKE2_F_ARG_LEN: usize = 213;
    const PROOF: &str = "Checked the length of the input above; qed";

    if input.len() != BLAKE2_F_ARG_LEN {
        panic!("input length for Blake2 F precompile should be exactly 213 bytes");
    }

    let mut cursor = Cursor::new(&input);
    let rounds = cursor.read_u32::<BigEndian>().expect(PROOF);

    // state vector, h
    let mut h = [0u64; 8];
    for state_word in &mut h {
        *state_word = cursor.read_u64::<LittleEndian>().expect(PROOF);
    }

    // message block vector, m
    let mut m = [0u64; 16];
    for msg_word in &mut m {
        *msg_word = cursor.read_u64::<LittleEndian>().expect(PROOF);
    }

    // 2w-bit offset counter, t
    let t = [
        cursor.read_u64::<LittleEndian>().expect(PROOF),
        cursor.read_u64::<LittleEndian>().expect(PROOF),
    ];

    // final block indicator flag, "f"
    let f = match input.last() {
            Some(1) => true,
            Some(0) => false,
            _ => {
                panic!("incorrect final block indicator flag, was: {:?}", input.last());
            }
        };

    compress(&mut h, m, t, f, rounds as usize);

    let mut output = [0u8; 64];

    let mut output_buf = [0u8; 64];
    for (i, state_word) in h.iter().enumerate() {
        output_buf[i*8..(i+1)*8].copy_from_slice(&state_word.to_le_bytes());
    }

    (&mut output[..]).write(&output_buf).unwrap();

    output
}

pub fn run_bn_add(input: &[u8]) -> [u8; 64] {
    use bn::{AffineG1};
    use std::io::Write;

    let p1 = helpers::read_point(&input[0..64]).unwrap();
    let p2 = helpers::read_point(&input[64..128]).unwrap();

    let mut write_buf = [0u8; 64];
    if let Some(sum) = AffineG1::from_jacobian(p1 + p2) {
        // point not at infinity
        sum.x().to_big_endian(&mut write_buf[0..32]).expect("Cannot fail since 0..32 is 32-byte length");
        sum.y().to_big_endian(&mut write_buf[32..64]).expect("Cannot fail since 32..64 is 32-byte length");
    }
    
    let mut output = [0u8; 64];
    (&mut output[..]).write(&write_buf).unwrap();

    output
}

pub fn run_bn_mul(input: &[u8]) -> [u8; 64] {
    use bn::{AffineG1};
    use std::io::Write;

    let p1 = helpers::read_point(&input[0..64]).unwrap();                 
    let fr = helpers::read_fr(&input[64..96]).unwrap();

    let mut write_buf = [0u8; 64];
    if let Some(sum) = AffineG1::from_jacobian(p1 * fr) {
        // point not at infinity
        sum.x().to_big_endian(&mut write_buf[0..32]).expect("Cannot fail since 0..32 is 32-byte length");
        sum.y().to_big_endian(&mut write_buf[32..64]).expect("Cannot fail since 32..64 is 32-byte length");
    }
    
    let mut output = [0u8; 64];
    (&mut output[..]).write(&write_buf).unwrap();

    output
}

pub fn run_bn_pair(input: &[u8]) -> [u8; 32] {
    use bn::{AffineG1, AffineG2, G1, G2, Group, Fq, Fq2, pairing_batch, Gt};
    use ethereum_types::U256;
    use std::io::Write;

    let mut output = [0u8; 32];

    let ret_val = if input.is_empty() {
        U256::one()
    } else {
        // (a, b_a, b_b - each 64-byte affine coordinates)
        let elements = input.len() / 192;
        let mut vals = Vec::new();
        for idx in 0..elements {
            let a_x = Fq::from_slice(&input[idx*192..idx*192+32])
                .map_err(|_| "Invalid a argument x coordinate").unwrap();

            let a_y = Fq::from_slice(&input[idx*192+32..idx*192+64])
                .map_err(|_| "Invalid a argument y coordinate").unwrap();

            let b_a_y = Fq::from_slice(&input[idx*192+64..idx*192+96])
                .map_err(|_| "Invalid b argument imaginary coeff x coordinate").unwrap();

            let b_a_x = Fq::from_slice(&input[idx*192+96..idx*192+128])
                .map_err(|_| "Invalid b argument imaginary coeff y coordinate").unwrap();

            let b_b_y = Fq::from_slice(&input[idx*192+128..idx*192+160])
                .map_err(|_| "Invalid b argument real coeff x coordinate").unwrap();

            let b_b_x = Fq::from_slice(&input[idx*192+160..idx*192+192])
                .map_err(|_| "Invalid b argument real coeff y coordinate").unwrap();

            let b_a = Fq2::new(b_a_x, b_a_y);
            let b_b = Fq2::new(b_b_x, b_b_y);
            let b = if b_a.is_zero() && b_b.is_zero() {
                G2::zero()
            } else {
                G2::from(AffineG2::new(b_a, b_b).map_err(|_| "Invalid b argument - not on curve").unwrap())
            };
            let a = if a_x.is_zero() && a_y.is_zero() {
                G1::zero()
            } else {
                G1::from(AffineG1::new(a_x, a_y).map_err(|_| "Invalid a argument - not on curve").unwrap())
            };
            vals.push((a, b));
        };

        let mul = pairing_batch(&vals);

        if mul == Gt::one() {
            U256::one()
        } else {
            U256::zero()
        }
    };

    let mut buf = [0u8; 32];
    ret_val.to_big_endian(&mut buf);
    (&mut output[..]).write(&buf).unwrap();

    output
}