pub fn read_fr(reader: &[u8]) -> Result<bn::Fr, &'static str> {
    let mut buf = [0u8; 32];
    buf.copy_from_slice(&reader);

    bn::Fr::from_slice(&buf[0..32]).map_err(|_| "Invalid field element")
}

pub fn read_point(reader: &[u8]) -> Result<bn::G1, &'static str> {
    use bn::{Fq, AffineG1, G1, Group};

    let mut buf = [0u8; 32];

    buf.copy_from_slice(&reader[0..32]);

    let px = Fq::from_slice(&buf[0..32]).map_err(|_| "Invalid point x coordinate")?;

    buf.copy_from_slice(&reader[32..64]);

    let py = Fq::from_slice(&buf[0..32]).map_err(|_| "Invalid point y coordinate")?;
    Ok(
        if px == Fq::zero() && py == Fq::zero() {
            G1::zero()
        } else {
            AffineG1::new(px, py).map_err(|_| "Invalid curve point")?.into()
        }
    )
}

pub fn encode_g1_point(p: bn::AffineG1) -> [u8; 64] {
    let mut output = [0u8; 64];
    p.x().to_big_endian(&mut output[0..32]).expect("Cannot fail since 0..32 is 32-byte length");
    p.y().to_big_endian(&mut output[32..64]).expect("Cannot fail since 32..64 is 32-byte length");

    output
}