extern crate reqwest;
use std::io;

fn main() {
    let mut res = reqwest::get("http://iost:30001/getNodeInfo").unwrap();
    res.copy_to(&mut io::stdout()).unwrap();
}
