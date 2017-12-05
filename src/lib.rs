extern crate libc;
use std::ffi::CString;
pub use msg::*;

mod msg {
	use super::*;
    #[no_mangle]
    pub extern fn new_msg () -> *mut libc::c_char {
        let quote = String::from("There there");

        let c_str_quote = CString::new(quote).unwrap();
        c_str_quote.into_raw()
    }

    #[no_mangle]
    pub extern fn free_msg(s: *mut libc::c_char) {
        unsafe {
            if s.is_null() { return }
            CString::from_raw(s)
        };
	}

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
