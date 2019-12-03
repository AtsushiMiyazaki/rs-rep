extern crate rand_core;
extern crate rand_os;
extern crate ed25519_dalek;
use std::vec::Vec;

use rand_core::{CryptoRng, RngCore};
use rand_os::OsRng;
use ed25519_dalek::Keypair;
use ed25519_dalek::Signature;


fn main (){
    let mut csprng: OsRng = OsRng::new().unwrap();
    let keypairs: Keypair = Keypair::generate(&mut csprng);
    let l =  keypairs.to_bytes();

    let converted: String = String::from_utf8(l.to_vec()).unwrap();

    println!("{}", converted);
}
