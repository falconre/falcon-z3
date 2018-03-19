mod ast;
mod config;
mod context;
mod model;
mod solver;
mod sort;
pub mod z3_sys;


pub use self::ast::Ast;
pub use self::config::Config;
pub use self::context::Context;
pub use self::model::Model;
pub use self::solver::{Check, Solver};
pub use self::sort::Sort;


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

    println!("{:?}", a7.to_str(&context));

    solver.assert(&a7);
    solver.assert(&context.bvult(&a, &b));

    println!("solver.check() = {:?}", solver.check());

    let model = Model::new(&context, &solver).unwrap();
    let b_value = model.get_const_interp(&b).unwrap();

    println!("b: {:?}", b_value.to_str(&context));
}