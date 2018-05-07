use std::os::raw::c_char;

extern {
    pub fn alert(ptr: *const c_char);
    pub fn log(ptr: *const c_char);
}
