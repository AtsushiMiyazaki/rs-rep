extern crate sha3;
extern crate bs58;


use sha3::{Sha3_256, Digest};

const BUFFER_SIZE: usize = 1024;

fn print_result(sum: &[u8]) {
    let v = bs58::encode(sum.to_vec()).into_string();

    print!("{:?}", v);
}


fn main() {
    let mut sh = Sha3_256::default();
    let mut v = vec!([34, 15, 148, 153, 132, 91, 252, 10, 143, 175, 187, 23, 204, 3, 32, 251, 234, 65, 140, 111, 154, 203, 196, 152, 199, 246, 251, 128, 163, 202, 42, 241, 167, 75, 172, 157, 193, 155, 41, 189, 190, 226, 73, 53, 238, 73, 88, 208, 190, 60, 99, 90, 76, 203, 245, 59, 47, 72, 119, 62, 231, 20, 114, 6]);
    
    for bytes in v {
        print!("{:?}", &bytes);
    }
    
    print_result(&sh.result());
}
