extern crate afl;
extern crate indy_crypto;

use indy_crypto::bls::SignKey;

fn main() {
    afl::read_stdio_bytes(|bytes| {
        let _ = SignKey::from_bytes(&bytes);
    });
}
