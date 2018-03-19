use Ast;
use Config;
use Sort;
use std::ffi::CString;
use z3_sys;



pub struct Context {
    pub(crate) context: z3_sys::Z3_context
}


impl Context {
    pub fn new(config: Config) -> Context {
        Context {
            context: unsafe { z3_sys::Z3_mk_context(config.config) }
        }
    }

    pub fn bvadd(&self, lhs: &Ast, rhs: &Ast) -> Ast {
        Ast {
            ast: unsafe { z3_sys::Z3_mk_bvadd(self.context, lhs.ast, rhs.ast) }
        }
    }

    pub fn bvand(&self, lhs: &Ast, rhs: &Ast) -> Ast {
        Ast {
            ast: unsafe { z3_sys::Z3_mk_bvand(self.context, lhs.ast, rhs.ast) }
        }
    }

    pub fn bvmul(&self, lhs: &Ast, rhs: &Ast) -> Ast {
        Ast {
            ast: unsafe { z3_sys::Z3_mk_bvmul(self.context, lhs.ast, rhs.ast) }
        }
    }

    pub fn bvor(&self, lhs: &Ast, rhs: &Ast) -> Ast {
        Ast {
            ast: unsafe { z3_sys::Z3_mk_bvor(self.context, lhs.ast, rhs.ast) }
        }
    }

    pub fn bvsdiv(&self, lhs: &Ast, rhs: &Ast) -> Ast {
        Ast {
            ast: unsafe { z3_sys::Z3_mk_bvsdiv(self.context, lhs.ast, rhs.ast) }
        }
    }

    pub fn bvshl(&self, lhs: &Ast, rhs: &Ast) -> Ast {
        Ast {
            ast: unsafe { z3_sys::Z3_mk_bvshl(self.context, lhs.ast, rhs.ast) }
        }
    }

    pub fn bvshr(&self, lhs: &Ast, rhs: &Ast) -> Ast {
        Ast {
            ast: unsafe { z3_sys::Z3_mk_bvlshr(self.context, lhs.ast, rhs.ast) }
        }
    }

    pub fn bvsle(&self, lhs: &Ast, rhs: &Ast) -> Ast {
        Ast {
            ast: unsafe { z3_sys::Z3_mk_bvsle(self.context, lhs.ast, rhs.ast) }
        }
    }

    pub fn bvslt(&self, lhs: &Ast, rhs: &Ast) -> Ast {
        Ast {
            ast: unsafe { z3_sys::Z3_mk_bvslt(self.context, lhs.ast, rhs.ast) }
        }
    }

    pub fn bvsrem(&self, lhs: &Ast, rhs: &Ast) -> Ast {
        Ast {
            ast: unsafe { z3_sys::Z3_mk_bvsrem(self.context, lhs.ast, rhs.ast) }
        }
    }

    pub fn bvsub(&self, lhs: &Ast, rhs: &Ast) -> Ast {
        Ast {
            ast: unsafe { z3_sys::Z3_mk_bvsub(self.context, lhs.ast, rhs.ast) }
        }
    }

    pub fn bvudiv(&self, lhs: &Ast, rhs: &Ast) -> Ast {
        Ast {
            ast: unsafe { z3_sys::Z3_mk_bvudiv(self.context, lhs.ast, rhs.ast) }
        }
    }

    pub fn bvule(&self, lhs: &Ast, rhs: &Ast) -> Ast {
        Ast {
            ast: unsafe { z3_sys::Z3_mk_bvult(self.context, lhs.ast, rhs.ast) }
        }
    }

    pub fn bvult(&self, lhs: &Ast, rhs: &Ast) -> Ast {
        Ast {
            ast: unsafe { z3_sys::Z3_mk_bvult(self.context, lhs.ast, rhs.ast) }
        }
    }

    pub fn bvurem(&self, lhs: &Ast, rhs: &Ast) -> Ast {
        Ast {
            ast: unsafe { z3_sys::Z3_mk_bvurem(self.context, lhs.ast, rhs.ast) }
        }
    }

    pub fn bvxor(&self, lhs: &Ast, rhs: &Ast) -> Ast {
        Ast {
            ast: unsafe { z3_sys::Z3_mk_bvxor(self.context, lhs.ast, rhs.ast) }
        }
    }

    pub fn concat(&self, t1: &Ast, t2: &Ast) -> Ast {
        Ast {
            ast: unsafe { z3_sys::Z3_mk_concat(self.context, t1.ast, t2.ast) }
        }
    }

    pub fn extract(&self, high: u32, low: u32, t1: &Ast) -> Ast {
        Ast {
            ast: unsafe { z3_sys::Z3_mk_extract(self.context,
                                                high,
                                                low,
                                                t1.ast) }
        }
    }

    pub fn eq(&self, lhs: &Ast, rhs: &Ast) -> Ast {
        Ast {
            ast: unsafe { z3_sys::Z3_mk_eq(self.context, lhs.ast, rhs.ast) }
        }
    }
    
    pub fn mk_bv_sort(&self, bits: usize) -> Sort {
        Sort {
            sort: unsafe { 
                z3_sys::Z3_mk_bv_sort(self.context, bits as u32)
            }
        }
    }

    pub fn mk_numeral(&self, n: u64, sort: &Sort)
        -> Result<Ast, ::std::ffi::NulError> {

        let num_string = CString::new(format!("{}", n))?;
        unsafe {
            Ok(Ast {
                ast: z3_sys::Z3_mk_numeral(self.context,
                                           num_string.as_ptr(),
                                           sort.sort)
            })
        }
    }

    pub fn mk_var<S: Into<String>>(&self, name: S, sort: &Sort)
        -> Result<Ast, ::std::ffi::NulError> {

        let name: CString = CString::new(name.into() as String)?;
        unsafe {
            let symbol = z3_sys::Z3_mk_string_symbol(self.context,
                                                     name.as_ptr());
            Ok(Ast { ast: z3_sys::Z3_mk_const(self.context,
                                              symbol,
                                              sort.sort) })
        }
    }
}


impl Drop for Context {
    fn drop(&mut self) {
        if !self.context.is_null() {
            unsafe { z3_sys::Z3_del_context(self.context) }
        }
    }
}