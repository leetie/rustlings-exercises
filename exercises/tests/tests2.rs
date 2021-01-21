// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

fn calculate_num() -> u32 {
    let num = 1 * 2 * 3 * 4 * 5;
    num
}
#[cfg(test)]
mod tests {
    use calculate_num;
    #[test]

    fn you_can_assert_eq() {
        // assert_eq!(calculate_num(), 120); passes
        // assert_eq!("Ed", "ed"); fails
    }
}
