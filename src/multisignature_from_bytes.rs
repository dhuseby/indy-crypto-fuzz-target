extern crate afl;
extern crate indy_crypto;

use indy_crypto::bls::MultiSignature;

fn main() {
    afl::read_stdio_bytes(|bytes| {
        let _ = MultiSignature::from_bytes(&bytes);
    });
}
