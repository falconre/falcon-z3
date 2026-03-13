use crate::error::{Error, Result};
use crate::{Ast, Context};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Check {
    Sat,
    Unknown,
    Unsat,
}

pub struct Solver<'c> {
    pub(crate) solver: z3_sys::Z3_solver,
    context: &'c Context,
}

impl<'c> Solver<'c> {
    pub fn new(context: &'c Context) -> Result<Solver<'c>> {
        let solver = unsafe { z3_sys::Z3_mk_solver(context.context) }
            .ok_or_else(|| Error::Z3("Z3_mk_solver returned null".into()))?;
        unsafe {
            z3_sys::Z3_solver_inc_ref(context.context, solver);
        }
        Ok(Solver { solver, context })
    }

    pub fn assert(&self, constraint: &Ast) {
        unsafe {
            z3_sys::Z3_solver_assert(self.context.context, self.solver, constraint.ast);
        }
    }

    pub fn check(&self) -> Check {
        let lbool = unsafe { z3_sys::Z3_solver_check(self.context.context, self.solver) };
        if lbool == z3_sys::Z3_L_FALSE {
            Check::Unsat
        } else if lbool == z3_sys::Z3_L_TRUE {
            Check::Sat
        } else {
            Check::Unknown
        }
    }
}

impl<'c> Drop for Solver<'c> {
    fn drop(&mut self) {
        unsafe { z3_sys::Z3_solver_dec_ref(self.context.context, self.solver) }
    }
}
