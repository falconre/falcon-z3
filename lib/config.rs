use std::ffi::CString;

use crate::error::{Error, Result};

pub struct Config {
    pub(crate) config: z3_sys::Z3_config,
}

impl Config {
    pub fn new() -> Result<Config> {
        let config = unsafe { z3_sys::Z3_mk_config() }
            .ok_or_else(|| Error::Z3("Z3_mk_config returned null".into()))?;
        Ok(Config { config })
    }

    pub fn enable_model(self) -> Result<Self> {
        let model = CString::new("model")?;
        let true_ = CString::new("true")?;
        unsafe {
            z3_sys::Z3_set_param_value(self.config, model.as_ptr(), true_.as_ptr());
        }
        Ok(self)
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        unsafe { z3_sys::Z3_del_config(self.config) }
    }
}
