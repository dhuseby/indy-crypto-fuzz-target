extern crate afl;
extern crate indy_crypto;

use indy_crypto::bls::VerKey;

fn main() {
    afl::read_stdio_bytes(|bytes| {
        let _ = VerKey::from_bytes(&bytes);
    });
}
