use std::ffi::CString;

use crate::error::{Error, Result};
use crate::{Ast, Config, Sort};

macro_rules! z3_binop {
    ($name:ident, $z3_fn:ident) => {
        pub fn $name(&self, lhs: &Ast, rhs: &Ast) -> Result<Ast> {
            let ast = unsafe { z3_sys::$z3_fn(self.context, lhs.ast, rhs.ast) }
                .ok_or_else(|| Error::Z3(concat!(stringify!($z3_fn), " returned null").into()))?;
            Ok(Ast { ast })
        }
    };
}

macro_rules! z3_unop {
    ($name:ident, $z3_fn:ident) => {
        pub fn $name(&self, t1: &Ast) -> Result<Ast> {
            let ast = unsafe { z3_sys::$z3_fn(self.context, t1.ast) }
                .ok_or_else(|| Error::Z3(concat!(stringify!($z3_fn), " returned null").into()))?;
            Ok(Ast { ast })
        }
    };
}

pub struct Context {
    pub(crate) context: z3_sys::Z3_context,
}

impl Context {
    pub fn new(config: Config) -> Result<Context> {
        let context = unsafe { z3_sys::Z3_mk_context(config.config) }
            .ok_or_else(|| Error::Z3("Z3_mk_context returned null".into()))?;
        Ok(Context { context })
    }

    z3_binop!(bvadd, Z3_mk_bvadd);
    z3_binop!(bvand, Z3_mk_bvand);
    z3_binop!(bvmul, Z3_mk_bvmul);
    z3_binop!(bvor, Z3_mk_bvor);
    z3_binop!(bvsdiv, Z3_mk_bvsdiv);
    z3_binop!(bvshl, Z3_mk_bvshl);
    z3_binop!(bvlshr, Z3_mk_bvlshr);
    z3_binop!(bvashr, Z3_mk_bvashr);
    z3_binop!(bvsle, Z3_mk_bvsle);
    z3_binop!(bvslt, Z3_mk_bvslt);
    z3_binop!(bvsrem, Z3_mk_bvsrem);
    z3_binop!(bvsub, Z3_mk_bvsub);
    z3_binop!(bvudiv, Z3_mk_bvudiv);
    z3_binop!(bvule, Z3_mk_bvule);
    z3_binop!(bvult, Z3_mk_bvult);
    z3_binop!(bvurem, Z3_mk_bvurem);
    z3_binop!(bvxor, Z3_mk_bvxor);
    z3_binop!(concat, Z3_mk_concat);
    z3_binop!(eq, Z3_mk_eq);

    z3_unop!(bvnot, Z3_mk_bvnot);
    z3_unop!(not, Z3_mk_not);

    pub fn extract(&self, high: u32, low: u32, t1: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_extract(self.context, high, low, t1.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_extract returned null".into()))?;
        Ok(Ast { ast })
    }

    pub fn ite(&self, condition: &Ast, then: &Ast, else_: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_ite(self.context, condition.ast, then.ast, else_.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_ite returned null".into()))?;
        Ok(Ast { ast })
    }

    pub fn mk_bv_sort(&self, bits: usize) -> Result<Sort> {
        let sort = unsafe { z3_sys::Z3_mk_bv_sort(self.context, bits as u32) }
            .ok_or_else(|| Error::Z3("Z3_mk_bv_sort returned null".into()))?;
        Ok(Sort { sort })
    }

    pub fn mk_numeral(&self, n: u64, sort: &Sort) -> Result<Ast> {
        let num_string = CString::new(format!("{}", n))?;
        let ast = unsafe { z3_sys::Z3_mk_numeral(self.context, num_string.as_ptr(), sort.sort) }
            .ok_or_else(|| Error::Z3("Z3_mk_numeral returned null".into()))?;
        Ok(Ast { ast })
    }

    pub fn mk_var<S: Into<String>>(&self, name: S, sort: &Sort) -> Result<Ast> {
        let name: CString = CString::new(name.into())?;
        let symbol = unsafe { z3_sys::Z3_mk_string_symbol(self.context, name.as_ptr()) }
            .ok_or_else(|| Error::Z3("Z3_mk_string_symbol returned null".into()))?;
        let ast = unsafe { z3_sys::Z3_mk_const(self.context, symbol, sort.sort) }
            .ok_or_else(|| Error::Z3("Z3_mk_const returned null".into()))?;
        Ok(Ast { ast })
    }

    /// Sign extend `rhs` by `i` additional bytes. To sign-extend a 50-bit value
    /// to a 60-bit value, `i` would be `10`.
    pub fn sign_ext(&self, i: u32, rhs: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_sign_ext(self.context, i, rhs.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_sign_ext returned null".into()))?;
        Ok(Ast { ast })
    }

    /// Zero extend `rhs` by `i` additional bytes. To zero-extend a 50-bit value
    /// to a 60-bit value, `i` would be `10`.
    pub fn zero_ext(&self, i: u32, rhs: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_zero_ext(self.context, i, rhs.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_zero_ext returned null".into()))?;
        Ok(Ast { ast })
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { z3_sys::Z3_del_context(self.context) }
    }
}
