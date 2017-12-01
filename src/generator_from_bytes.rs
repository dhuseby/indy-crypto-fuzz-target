extern crate afl;
extern crate indy_crypto;

use indy_crypto::bls::Generator;

fn main() {
    afl::read_stdio_bytes(|bytes| {
        let _ = Generator::from_bytes(&bytes);
    });
}
