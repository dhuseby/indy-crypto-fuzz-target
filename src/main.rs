extern crate afl;
extern crate indy_crypto;

fn main() {
    afl::read_stdio_bytes(|bytes| {
        indy_crypto::bls::SignKey::new(Some(&bytes)).unwrap();
    });
}
