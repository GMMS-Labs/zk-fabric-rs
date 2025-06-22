# Overview : 1-out-of-2 OT Using XOR Masking

- Alice has two keys: k0, k1.
- Bob selects a bit b ∈ {0, 1}.
- Bob sends a choice commitment (like a hash or seed).
- Alice masks both keys with PRF(seed, 0) and PRF(seed, 1) and sends both masked keys
- Bob derives PRF(seed, b) and unmasks only the key he wants.

We can simulate this with:

- A shared seed.
- A deterministic PRF (HMAC-SHA256 or SHA256(seed || b)).
- XOR masking.

## Step-by-Step

1. Add dependencies
   Make sure you have these in your Cargo.toml:

```
[dependencies]
rand = "0.8"
sha2 = "0.10"
hmac = "0.12"
```

2. ![OT Implementation](src/xor-masked-ot.rs)

3. Test Example

```
mod xor_masked_ot;

use rand::Rng;

fn main() {
    let seed: Vec<u8> = (0..16).map(|_| rand::random::<u8>()).collect();

    let k0: Vec<u8> = (0..16).map(|_| rand::random::<u8>()).collect(); // key for bit 0
    let k1: Vec<u8> = (0..16).map(|_| rand::random::<u8>()).collect(); // key for bit 1

    // Bob chooses a bit (0 or 1)
    let b = 1u8;

    // Alice prepares masked keys
    let (c0, c1) = xor_masked_ot::dummy_ot_send(&seed, &k0, &k1);

    // Bob receives his key
    let recovered = xor_masked_ot::dummy_ot_receive(&seed, b, &c0, &c1);

    let expected = if b == 0 { k0 } else { k1 };
    assert_eq!(recovered, expected);

    println!("✅ Dummy OT works! Bob received the correct key.");
}
```

## Integration with existing codebase

- Replace direct wire key selection with dummy_ot_receive.
- Alice uses dummy_ot_send to prepare ciphertexts for each wire.
- Bob uses his random seed and bit to recover only the desired key.

### Optional Enhancements

- Feature Suggestion
- Hiding Seed Hash `seed
- Commitment Bob hashes seed and publishes it (Alice can't invert).
- DLT Store Publish (wire_id, c0, c1) to a file or log.
