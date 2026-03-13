use std::ffi::CString;

use crate::error::{Error, Result};
use crate::{Ast, Config, Sort};

pub struct Context {
    pub(crate) context: z3_sys::Z3_context,
}

impl Context {
    pub fn new(config: Config) -> Result<Context> {
        let context = unsafe { z3_sys::Z3_mk_context(config.config) }
            .ok_or_else(|| Error::Z3("Z3_mk_context returned null".into()))?;
        Ok(Context { context })
    }

    pub fn bvadd(&self, lhs: &Ast, rhs: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_bvadd(self.context, lhs.ast, rhs.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_bvadd returned null".into()))?;
        Ok(Ast { ast })
    }

    pub fn bvand(&self, lhs: &Ast, rhs: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_bvand(self.context, lhs.ast, rhs.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_bvand returned null".into()))?;
        Ok(Ast { ast })
    }

    pub fn bvmul(&self, lhs: &Ast, rhs: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_bvmul(self.context, lhs.ast, rhs.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_bvmul returned null".into()))?;
        Ok(Ast { ast })
    }

    pub fn bvnot(&self, t1: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_bvnot(self.context, t1.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_bvnot returned null".into()))?;
        Ok(Ast { ast })
    }

    pub fn bvor(&self, lhs: &Ast, rhs: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_bvor(self.context, lhs.ast, rhs.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_bvor returned null".into()))?;
        Ok(Ast { ast })
    }

    pub fn bvsdiv(&self, lhs: &Ast, rhs: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_bvsdiv(self.context, lhs.ast, rhs.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_bvsdiv returned null".into()))?;
        Ok(Ast { ast })
    }

    pub fn bvshl(&self, lhs: &Ast, rhs: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_bvshl(self.context, lhs.ast, rhs.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_bvshl returned null".into()))?;
        Ok(Ast { ast })
    }

    pub fn bvlshr(&self, lhs: &Ast, rhs: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_bvlshr(self.context, lhs.ast, rhs.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_bvlshr returned null".into()))?;
        Ok(Ast { ast })
    }

    pub fn bvashr(&self, lhs: &Ast, rhs: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_bvashr(self.context, lhs.ast, rhs.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_bvashr returned null".into()))?;
        Ok(Ast { ast })
    }

    pub fn bvsle(&self, lhs: &Ast, rhs: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_bvsle(self.context, lhs.ast, rhs.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_bvsle returned null".into()))?;
        Ok(Ast { ast })
    }

    pub fn bvslt(&self, lhs: &Ast, rhs: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_bvslt(self.context, lhs.ast, rhs.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_bvslt returned null".into()))?;
        Ok(Ast { ast })
    }

    pub fn bvsrem(&self, lhs: &Ast, rhs: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_bvsrem(self.context, lhs.ast, rhs.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_bvsrem returned null".into()))?;
        Ok(Ast { ast })
    }

    pub fn bvsub(&self, lhs: &Ast, rhs: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_bvsub(self.context, lhs.ast, rhs.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_bvsub returned null".into()))?;
        Ok(Ast { ast })
    }

    pub fn bvudiv(&self, lhs: &Ast, rhs: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_bvudiv(self.context, lhs.ast, rhs.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_bvudiv returned null".into()))?;
        Ok(Ast { ast })
    }

    pub fn bvule(&self, lhs: &Ast, rhs: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_bvule(self.context, lhs.ast, rhs.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_bvule returned null".into()))?;
        Ok(Ast { ast })
    }

    pub fn bvult(&self, lhs: &Ast, rhs: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_bvult(self.context, lhs.ast, rhs.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_bvult returned null".into()))?;
        Ok(Ast { ast })
    }

    pub fn bvurem(&self, lhs: &Ast, rhs: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_bvurem(self.context, lhs.ast, rhs.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_bvurem returned null".into()))?;
        Ok(Ast { ast })
    }

    pub fn bvxor(&self, lhs: &Ast, rhs: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_bvxor(self.context, lhs.ast, rhs.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_bvxor returned null".into()))?;
        Ok(Ast { ast })
    }

    pub fn concat(&self, t1: &Ast, t2: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_concat(self.context, t1.ast, t2.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_concat returned null".into()))?;
        Ok(Ast { ast })
    }

    pub fn extract(&self, high: u32, low: u32, t1: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_extract(self.context, high, low, t1.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_extract returned null".into()))?;
        Ok(Ast { ast })
    }

    pub fn eq(&self, lhs: &Ast, rhs: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_eq(self.context, lhs.ast, rhs.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_eq returned null".into()))?;
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

    pub fn not(&self, a: &Ast) -> Result<Ast> {
        let ast = unsafe { z3_sys::Z3_mk_not(self.context, a.ast) }
            .ok_or_else(|| Error::Z3("Z3_mk_not returned null".into()))?;
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
