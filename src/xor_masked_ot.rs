use sha2::{Digest, Sha256};

fn prf(seed: &[u8], bit: u8) -> [u8; 16] {
    let mut hasher = Sha256::new();
    hasher.update(seed);
    hasher.update(&[bit]);
    let result = hasher.finalize();
    let mut output = [0u8; 16];
    output.copy_from_slice(&result[..16]); // Truncate to 128-bit output
    output
}

fn xor_mask(k: &[u8], prf_output: &[u8]) -> Vec<u8> {
    k.iter().zip(prf_output.iter()).map(|(a, b)| a ^ b).collect()
}

pub fn dummy_ot_send(seed: &[u8], k0: &[u8], k1: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let pad0 = prf(seed, 0);
    let pad1 = prf(seed, 1);
    let c0 = xor_mask(k0, &pad0);
    let c1 = xor_mask(k1, &pad1);
    (c0, c1)
}

pub fn dummy_ot_receive(seed: &[u8], b: u8, c0: &[u8], c1: &[u8]) -> Vec<u8> {
    let pad = prf(seed, b);
    let ciphertext = if b == 0 { c0 } else { c1 };
    xor_mask(ciphertext, &pad)
}
