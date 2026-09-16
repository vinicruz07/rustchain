#![cfg_attr(not(feature = "std"), no_std)]

pub fn hello_world() -> &'static str {
    "Hello from blockchain-core!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(hello_world(), "Hello from blockchain-core!");
    }
}
