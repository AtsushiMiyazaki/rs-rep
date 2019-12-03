#[cfg(test)]
mod tests {
    use super::iost;
    #[test]
    fn it_works() {
        let i = iost::init();
        assert_eq!(i.options.server, "127.0.0.1");
    }
    
}

pub mod iost;