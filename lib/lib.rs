#[macro_use]
extern crate error_chain;
extern crate falcon;
extern crate num_bigint;
extern crate num_traits;
extern crate z3_sys;

mod ast;
mod config;
mod context;
pub mod il;
mod model;
mod optimize;
mod solver;
mod sort;

pub use self::ast::Ast;
pub use self::config::Config;
pub use self::context::Context;
pub use self::model::Model;
pub use self::optimize::Optimize;
pub use self::solver::{Check, Solver};
pub use self::sort::Sort;

pub mod error {
    error_chain! {
        types {
            Error, ErrorKind, ResultExt, Result;
        }
        foreign_links {
            Falcon(::falcon::error::Error);
            NulError(::std::ffi::NulError);
        }
    }
}

#[test]
fn test() {
    let config = Config::new().enable_model();
    let context = Context::new(config);
    let solver = Solver::new(&context);

    let sort32 = context.mk_bv_sort(32);
    let a = context.mk_var("a", &sort32).unwrap();
    let b = context.mk_var("b", &sort32).unwrap();
    let seven = context.mk_numeral(7, &sort32).unwrap();

    let a7 = context.eq(&a, &seven);

    solver.assert(&a7);
    solver.assert(&context.bvult(&a, &b));

    let model = Model::new(&context, &solver).unwrap();
    let b_value = model.get_const_interp(&b).unwrap();

    println!("b: {:?}", b_value.to_string(&context));
}

#[cfg(test)]
use error::*;

#[test]
fn rdx() -> Result<()> {
    use falcon::il;

    let rdx = il::expr_scalar("rdx", 64);

    let constraint0 = il::Expression::cmpneq(il::expr_const(0, 64), il::expr_scalar("rdx", 64))?;

    let constraint1 = il::Expression::cmpneq(
        il::expr_const(0xffffffff_ffffffff, 64),
        il::expr_scalar("rdx", 64),
    )?;

    let constraint2 = il::Expression::cmpeq(
        il::Expression::cmpeq(
            il::Expression::cmpeq(
                il::Expression::shr(
                    il::Expression::sub(
                        il::Expression::add(
                            il::Expression::mul(il::expr_scalar("rdx", 64), il::expr_const(1, 64))?,
                            il::expr_const(0x7FFFFFFFFF, 64),
                        )?,
                        il::expr_const(1, 64),
                    )?,
                    il::expr_const(0x30, 64),
                )?,
                il::expr_const(0, 64),
            )?,
            il::expr_const(0, 1),
        )?,
        il::expr_const(0, 1),
    )?;

    let constraints = vec![constraint0, constraint1, constraint2];

    let rdx_const = self::il::solve(&constraints, &rdx).unwrap().unwrap();

    println!("{}", rdx_const);

    assert!(rdx_const.value_u64().unwrap() != 0);

    Ok(())
}
