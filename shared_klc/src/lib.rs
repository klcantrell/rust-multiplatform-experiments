uniffi::setup_scaffolding!("shared");

#[uniffi::export]
pub fn add_stuff(left: i32, right: i32) -> i32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_stuff(2, 2);
        assert_eq!(result, 4);
    }
}
