#[macro_use] extern crate error_chain;
extern crate falcon;
extern crate num_bigint;
extern crate num_traits;
extern crate z3_sys;

mod ast;
mod config;
mod context;
pub mod il;
mod model;
mod solver;
mod sort;


pub use self::ast::Ast;
pub use self::config::Config;
pub use self::context::Context;
pub use self::model::Model;
pub use self::solver::{Check, Solver};
pub use self::sort::Sort;



pub mod error {
    error_chain! {
        types {
            Error, ErrorKind, ResultExt, Result;
        }
        foreign_links {
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

    println!("b: {:?}", b_value.to_str(&context));
}