extern crate afl;
extern crate indy_crypto;

use indy_crypto::bls::{Generator, SignKey, VerKey, Signature, MultiSignature};

fn main() {
    afl::read_stdio_bytes(|bytes| {
        //let _ = Generator::from_bytes(&bytes);
        let _ = SignKey::new(Some(&bytes));
        let _ = SignKey::from_bytes(&bytes);
        //let _ = VerKey::from_bytes(&bytes);
        let _ = Signature::from_bytes(&bytes);
        let _ = MultiSignature::from_bytes(&bytes);
    });
}
