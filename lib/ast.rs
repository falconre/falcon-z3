use std::ffi::CStr;

use crate::Context;

pub struct Ast {
    pub(crate) ast: z3_sys::Z3_ast,
}

fn z3_str_to_string(s: *const std::os::raw::c_char) -> Option<String> {
    if s.is_null() {
        None
    } else {
        unsafe { CStr::from_ptr(s) }
            .to_str()
            .ok()
            .map(|s| s.to_string())
    }
}

impl Ast {
    pub fn to_string(&self, context: &Context) -> Option<String> {
        z3_str_to_string(unsafe { z3_sys::Z3_ast_to_string(context.context, self.ast) })
    }

    pub fn get_numeral_decimal_string(&self, context: &Context) -> Option<String> {
        z3_str_to_string(unsafe {
            z3_sys::Z3_get_numeral_decimal_string(context.context, self.ast, 10000)
        })
    }
}
