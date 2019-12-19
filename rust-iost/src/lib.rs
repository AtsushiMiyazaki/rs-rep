pub mod iost;


#[cfg(test)]
mod tests {
    use super::iost;
    use bs58;
    use ed25519_dalek::Signature;
    use std::str;
    #[test]
    fn it_works() {
        let k = iost::IOST::send();
    }
    
}
