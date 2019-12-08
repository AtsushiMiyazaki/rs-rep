pub mod iost;


#[cfg(test)]
mod tests {
    use super::iost;
    use bs58;
    #[test]
    fn it_works() {
        let i = iost::IOST::dec();
        println!("{:?}", i);
        let s = bs58::decode("2yquS3ySrGWPEKywCPzX4RTJugqRh7kJSo5aehsLYPEWkUxBWA39oMrZ7ZxuM4fgyXYs2cPwh5n8aNNpH5x2VyK1").into_vec().unwrap();
        assert_eq!(i, );
    }
    
}
