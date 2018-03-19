use std::ffi::CString;
use z3_sys;


pub struct Config {
    pub(crate) config: z3_sys::Z3_config
}


impl Config {
    pub fn new() -> Config {
        Config {
            config: unsafe { z3_sys::Z3_mk_config() }
        }
    }

    pub fn enable_model(self: Self) -> Self {
        let model = CString::new("model").unwrap();
        let true_ = CString::new("true").unwrap();
        unsafe {
            z3_sys::Z3_set_param_value(self.config,
                                       model.as_ptr(),
                                       true_.as_ptr());
        }
        self
    }
}


impl Drop for Config {
    fn drop(&mut self) {
        if !self.config.is_null() {
            unsafe { z3_sys::Z3_del_config(self.config) }
        }
    }
}