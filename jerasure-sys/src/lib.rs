#[allow(non_camel_case_types)]
mod bind_sys;
pub use bind_sys::*;

#[cfg(test)]
mod test {
    use crate::bind_sys;

    #[test]
    fn test_multiply() {
        unsafe {
            assert_eq!(bind_sys::galois_single_multiply(48, 18, 8), 71_i32);
        }
    }
}
