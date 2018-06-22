use Ast;
use Check;
use Context;
use Solver;
use std::ptr::null_mut;
use z3_sys;

pub struct Model<'c> {
    pub(crate) model: z3_sys::Z3_model,
    context: &'c Context
}


impl<'c> Model<'c> {
    pub fn new(context: &'c Context, solver: &Solver) -> Option<Model<'c>> {
        if solver.check() != Check::Sat {
            None
        }
        else {
            let m = unsafe {
                z3_sys::Z3_solver_get_model(context.context, solver.solver)
            };
            let model = Model { model: m, context: context };
            model.inc_ref();
            Some(model)
        }
    }


    pub fn get_const_interp(&self, t: &Ast) -> Option<Ast> {
        let mut ast: z3_sys::Z3_ast = null_mut();
        let r = unsafe {
            z3_sys::Z3_model_eval(
                self.context.context,
                self.model,
                t.ast,
                true,
                &mut ast as *mut z3_sys::Z3_ast)
        };
        if r {
            Some(Ast{ ast: ast })
        }
        else {
            None
        }
    }


    fn inc_ref(&self) {
        unsafe { z3_sys::Z3_model_inc_ref(self.context.context, self.model); }
    }

    fn dec_ref(&self) {
        unsafe { z3_sys::Z3_model_dec_ref(self.context.context, self.model); }
    }
}


impl<'c> Drop for Model<'c> {
    fn drop(&mut self) {
        self.dec_ref();
    }
}