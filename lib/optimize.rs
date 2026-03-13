use crate::error::{Error, Result};
use crate::{Ast, Check, Context};

pub struct Optimize<'c> {
    pub(crate) optimize: z3_sys::Z3_optimize,
    context: &'c Context,
}

impl<'c> Optimize<'c> {
    pub fn new(context: &'c Context) -> Result<Optimize<'c>> {
        let optimize = unsafe { z3_sys::Z3_mk_optimize(context.context) }
            .ok_or_else(|| Error::Z3("Z3_mk_optimize returned null".into()))?;
        unsafe {
            z3_sys::Z3_optimize_inc_ref(context.context, optimize);
        }
        Ok(Optimize { optimize, context })
    }

    pub fn assert(&self, term: &Ast) {
        unsafe {
            z3_sys::Z3_optimize_assert(self.context.context, self.optimize, term.ast);
        }
    }

    pub fn check(&self) -> Check {
        let lbool = unsafe {
            z3_sys::Z3_optimize_check(self.context.context, self.optimize, 0, std::ptr::null())
        };
        match lbool {
            z3_sys::Z3_L_FALSE => Check::Unsat,
            z3_sys::Z3_L_TRUE => Check::Sat,
            _ => Check::Unknown,
        }
    }

    pub fn maximize(&self, term: &Ast) {
        unsafe {
            z3_sys::Z3_optimize_maximize(self.context.context, self.optimize, term.ast);
        }
    }

    pub fn minimize(&self, term: &Ast) {
        unsafe { z3_sys::Z3_optimize_minimize(self.context.context, self.optimize, term.ast) };
    }
}

impl<'c> Drop for Optimize<'c> {
    fn drop(&mut self) {
        unsafe { z3_sys::Z3_optimize_dec_ref(self.context.context, self.optimize) }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::{Config, Context, Model, Optimize};

    #[test]
    fn test_maximize() -> Result<()> {
        let config = Config::new()?.enable_model()?;
        let context = Context::new(config)?;
        let optimize = Optimize::new(&context)?;

        let sort32 = context.mk_bv_sort(32)?;
        let a = context.mk_var("a", &sort32)?;
        let seven = context.mk_numeral(7, &sort32)?;

        optimize.assert(&context.bvult(&a, &seven)?);
        optimize.maximize(&a);

        let model = Model::new_optimize(&context, &optimize).unwrap();
        let a_value = model.get_const_interp(&a).unwrap();

        let a_value_string = a_value.get_numeral_decimal_string(&context).unwrap();

        println!("a_value_string {}", a_value_string);

        let a_value: usize = a_value_string.parse::<usize>().unwrap();

        assert_eq!(a_value, 6);
        Ok(())
    }

    #[test]
    fn test_minimize() -> Result<()> {
        let config = Config::new()?.enable_model()?;
        let context = Context::new(config)?;
        let optimize = Optimize::new(&context)?;

        let sort32 = context.mk_bv_sort(32)?;
        let a = context.mk_var("a", &sort32)?;
        let seven = context.mk_numeral(7, &sort32)?;

        optimize.assert(&context.bvult(&seven, &a)?);
        optimize.minimize(&a);

        let model = Model::new_optimize(&context, &optimize).unwrap();
        let a_value = model.get_const_interp(&a).unwrap();

        let a_value_string = a_value.get_numeral_decimal_string(&context).unwrap();

        println!("b_value_string {}", a_value_string);

        let a_value: usize = a_value_string.parse::<usize>().unwrap();

        assert_eq!(a_value, 8);
        Ok(())
    }
}
