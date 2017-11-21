extern crate afl;
extern crate indy_crypto;

fn main() {
    afl::read_stdio_bytes(|bytes| {
        indy_crypto::Signkey::new(Some(&bytes)).unwrap();
    });
}
