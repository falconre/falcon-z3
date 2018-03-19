use Context;
use std::ffi::CStr;
use z3_sys;



pub struct Ast {
    pub(crate) ast: z3_sys::Z3_ast
}


impl Ast {
    pub fn to_str(&self, context: &Context) -> Option<String> {
        let s = unsafe {
            z3_sys::Z3_ast_to_string(context.context, self.ast)
        };
        if s.is_null() {
            None
        }
        else {
            let cs = unsafe { CStr::from_ptr(s as (*mut i8)) };
            cs.to_str().ok().map(|s| s.to_string())
        }
    }
}