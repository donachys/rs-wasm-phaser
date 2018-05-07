use std::ffi::CString;

mod imports;

pub fn alert(s: &str) {
    let cstr = CString::new(s).unwrap();
    unsafe {
        imports::alert(cstr.as_ptr());
    }
}

pub fn log(s: &str) {
    let cstr = CString::new(s).unwrap();
    unsafe {
        imports::log(cstr.as_ptr());
    }
}
