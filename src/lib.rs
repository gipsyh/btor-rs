use aig::Aig;
use std::{
    ffi::{c_char, CString},
    os::raw::c_void,
};

extern "C" {
    fn btor2aiger(filename: *const c_char) -> *mut c_void;
}

pub fn btor_to_aiger(f: &str) -> Aig {
    let f = CString::new(f).unwrap();
    let aiger = unsafe { btor2aiger(f.as_ptr()) };
    Aig::from_aiger(aiger)
}
