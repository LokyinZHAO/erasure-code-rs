fn multiply<I: Into<::std::os::raw::c_int>>(a: I, b: I) -> i32 {
    let res = unsafe { jerasure_sys::galois_single_multiply(a.into(), b.into(), 8) };
    return res;
}

#[cfg(test)]
mod test {
    #[test]
    fn test_multiply() {
        assert_eq!(super::multiply(48, 18), 71_i32);
    }
}
