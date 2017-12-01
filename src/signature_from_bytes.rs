extern crate afl;
extern crate indy_crypto;

use indy_crypto::bls::Signature;

fn main() {
    afl::read_stdio_bytes(|bytes| {
        let _ = Signature::from_bytes(&bytes);
    });
}
