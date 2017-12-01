extern crate indy_crypto;

use std::fs::{create_dir, OpenOptions};
use std::io::{Result, Write};
use indy_crypto::bls::{Generator, SignKey, VerKey, MultiSignature, Bls};

fn create_dir_if_missing(path: &str) -> Result<()> {
    let dir = env!("CARGO_MANIFEST_DIR").to_owned() + "/" + path;
    match create_dir(&dir) {
        Ok(_) => println!("cargo:warning=Created {} dir", path),
        _ => {}, // ignore the error
    }
    Ok(())
}

fn create_fuzz_dirs_if_missing() -> Result<()> {
    let dirs = vec!["in", "out"];
    for dir in dirs {
        match create_dir_if_missing(&dir) {
            _ => {}, // ignore everything
        }
    }
    Ok(())
}

fn create_test_dirs_if_missing(name: &str) -> Result<()> {
    let dirs = vec!["in", "out"];
    for dir in dirs {
        let d = dir.to_owned() + "/" + name;
        match create_dir_if_missing(&d) {
            _ => {}, // ignore everything
        }
    }
    Ok(())
}

fn write_seed(name: &str, seed: &[u8]) -> Result<()> {
    create_test_dirs_if_missing(name)?;
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/in/").to_owned() + name + "/seed";
    let mut file = match OpenOptions::new().write(true).create(false).open(&path) {
        Ok(f) => f,
        Err(_) => return Ok(()),
    };
    let usize = file.write(seed)?;
    println!("cargo:warning=Generated {} with {} bytes", path, usize);
    Ok(())
}

fn main() {
    // set up the in and out dirs
    create_fuzz_dirs_if_missing().unwrap();

    // generate initial seed for Generator::from_bytes()
    let gen = Generator::new().unwrap();
    write_seed("generator-from-bytes", gen.as_bytes()).unwrap();
  
    // generate initial seed for SignKey::from_bytes()
    let sk = SignKey::new(None).unwrap();
    write_seed("signkey-from-bytes", sk.as_bytes()).unwrap();

    // generate initial seed for VerKey::from_bytes()
    let vk = VerKey::new(&gen, &sk).unwrap();
    write_seed("verkey-from-bytes", vk.as_bytes()).unwrap();

    // generate initial seed for Signature::from_bytes()
    let msg = vec![];
    let sig = Bls::sign(&msg, &sk).unwrap();
    write_seed("signature-from-bytes", sig.as_bytes()).unwrap();

    // generate initial seed for MultiSignature::from_bytes()
    let sigs = vec![&sig];
    let msig = MultiSignature::new(&sigs).unwrap();
    write_seed("multisignature-from-bytes", msig.as_bytes()).unwrap();
}
