pub fn hello() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_does_not_panic() {
        // Test that hello() runs without panicking
        hello();
    }
}
