use std::mem::MaybeUninit;

use crate::{Ast, Check, Context, Optimize, Solver};

pub struct Model<'c> {
    pub(crate) model: z3_sys::Z3_model,
    context: &'c Context,
}

impl<'c> Model<'c> {
    pub fn new(context: &'c Context, solver: &Solver) -> Option<Model<'c>> {
        if solver.check() != Check::Sat {
            return None;
        }
        let m = unsafe { z3_sys::Z3_solver_get_model(context.context, solver.solver) }?;
        let model = Model { model: m, context };
        model.inc_ref();
        Some(model)
    }

    pub fn new_optimize(context: &'c Context, optimize: &Optimize) -> Option<Model<'c>> {
        if optimize.check() != Check::Sat {
            return None;
        }
        let m = unsafe { z3_sys::Z3_optimize_get_model(context.context, optimize.optimize) }?;
        let model = Model { model: m, context };
        model.inc_ref();
        Some(model)
    }

    pub fn get_const_interp(&self, t: &Ast) -> Option<Ast> {
        let mut ast = MaybeUninit::<z3_sys::Z3_ast>::uninit();
        let r = unsafe {
            z3_sys::Z3_model_eval(
                self.context.context,
                self.model,
                t.ast,
                true,
                ast.as_mut_ptr(),
            )
        };
        if r {
            Some(Ast {
                ast: unsafe { ast.assume_init() },
            })
        } else {
            None
        }
    }

    fn inc_ref(&self) {
        unsafe {
            z3_sys::Z3_model_inc_ref(self.context.context, self.model);
        }
    }

    fn dec_ref(&self) {
        unsafe {
            z3_sys::Z3_model_dec_ref(self.context.context, self.model);
        }
    }
}

impl<'c> Drop for Model<'c> {
    fn drop(&mut self) {
        self.dec_ref();
    }
}
