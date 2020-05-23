use std::convert::TryInto;
use std::env;
use std::ffi::CString;
use std::os::raw::{c_void, c_char, c_int};
use std::ptr;

// unix only, but so is wine I think? figure out a windows story for this
use std::os::unix::ffi::OsStrExt;

extern "C" {
    fn wine_init_argv0_path(argv0: *const c_char);
    fn wine_init(argc: c_int, argv: *const *const c_char, error: *mut c_char, error_size: c_int);
}

fn main() {
    // loop {}

    unsafe {
        let argv0 = CString::new(env::args_os().nth(0).unwrap().as_bytes()).unwrap();
        let argv = [argv0.as_ptr(), argv0.as_ptr()];

        let mut error = [0 as c_char; 128];

        // wine_init_argv0_path(argv0.as_ptr());
        wine_init(argv.len().try_into().unwrap(), argv.as_ptr(), error.as_mut_ptr(), error.len().try_into().unwrap());
    }
}
