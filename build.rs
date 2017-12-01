extern crate indy_crypto;

use indy_crypto::pair::PointG2;

fn main() {
    let point = PointG2::new().unwrap();
    let s8d = point.to_bytes().unwrap();
    println!("PointG2 size: {}", s8d.len());
}
