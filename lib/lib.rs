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
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error(transparent)]
        Falcon(#[from] falcon::Error),
        #[error(transparent)]
        NulError(#[from] std::ffi::NulError),
        #[error("Z3 operation failed: {0}")]
        Z3(String),
    }

    pub type Result<T> = std::result::Result<T, Error>;
}

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::il as z3_il;
    use crate::{Check, Config, Context, Model, Solver};
    use falcon::il;

    #[test]
    fn test() -> Result<()> {
        let config = Config::new()?.enable_model()?;
        let context = Context::new(config)?;
        let solver = Solver::new(&context)?;

        let sort32 = context.mk_bv_sort(32)?;
        let a = context.mk_var("a", &sort32)?;
        let b = context.mk_var("b", &sort32)?;
        let seven = context.mk_numeral(7, &sort32)?;

        let a7 = context.eq(&a, &seven)?;

        solver.assert(&a7);
        solver.assert(&context.bvult(&a, &b)?);

        let model = Model::new(&context, &solver).unwrap();
        let b_value = model.get_const_interp(&b).unwrap();

        println!("b: {:?}", b_value.to_string(&context));
        Ok(())
    }

    #[test]
    fn rdx() -> Result<()> {
        let rdx = il::expr_scalar("rdx", 64);

        let constraint0 =
            il::Expression::cmpneq(il::expr_const(0, 64), il::expr_scalar("rdx", 64))?;

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
                                il::Expression::mul(
                                    il::expr_scalar("rdx", 64),
                                    il::expr_const(1, 64),
                                )?,
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

        let rdx_const = z3_il::solve(&constraints, &rdx)?.unwrap();

        println!("{}", rdx_const);

        assert!(rdx_const.value_u64().unwrap() != 0);

        Ok(())
    }

    #[test]
    fn test_bvule_correctness() -> Result<()> {
        let config = Config::new()?.enable_model()?;
        let context = Context::new(config)?;

        let sort8 = context.mk_bv_sort(8)?;
        let x = context.mk_var("x", &sort8)?;

        // bvule(x, x) should always be SAT (x <= x is always true)
        let solver_le = Solver::new(&context)?;
        solver_le.assert(&context.bvule(&x, &x)?);
        assert_eq!(solver_le.check(), Check::Sat);

        // bvult(x, x) should always be UNSAT (x < x is never true)
        let solver_lt = Solver::new(&context)?;
        solver_lt.assert(&context.bvult(&x, &x)?);
        assert_eq!(solver_lt.check(), Check::Unsat);

        Ok(())
    }

    #[test]
    fn test_bvashr() -> Result<()> {
        let config = Config::new()?.enable_model()?;
        let context = Context::new(config)?;
        let solver = Solver::new(&context)?;

        let sort8 = context.mk_bv_sort(8)?;
        // -128 in 8-bit signed = 0x80
        let val = context.mk_numeral(0x80, &sort8)?;
        // shift right by 1
        let shift = context.mk_numeral(1, &sort8)?;
        let result_var = context.mk_var("result", &sort8)?;

        // result == ashr(0x80, 1) should be 0xC0 (-64 in signed)
        let ashr_result = context.bvashr(&val, &shift)?;
        solver.assert(&context.eq(&result_var, &ashr_result)?);

        let model = Model::new(&context, &solver).unwrap();
        let result_value = model.get_const_interp(&result_var).unwrap();
        let result_str = result_value.get_numeral_decimal_string(&context).unwrap();
        let result_num: u64 = result_str.parse().unwrap();

        // 0x80 >> 1 (arithmetic) = 0xC0 = 192
        assert_eq!(result_num, 192);

        Ok(())
    }

    #[test]
    fn test_ashr_expression() -> Result<()> {
        let expr = il::Expression::ashr(il::expr_const(0x80, 8), il::expr_const(1, 8))?;

        let result = z3_il::solve(&[], &expr)?.unwrap();
        assert_eq!(result.value_u64().unwrap(), 192);

        Ok(())
    }

    #[test]
    fn test_solve_multi() -> Result<()> {
        use std::collections::HashMap;

        let a_expr = il::expr_scalar("a", 32);
        let b_expr = il::expr_scalar("b", 32);

        // a + b == 10, a == 3
        let constraint1 = il::Expression::cmpeq(
            il::Expression::add(il::expr_scalar("a", 32), il::expr_scalar("b", 32))?,
            il::expr_const(10, 32),
        )?;
        let constraint2 = il::Expression::cmpeq(il::expr_scalar("a", 32), il::expr_const(3, 32))?;

        let mut values = HashMap::new();
        values.insert("a".to_string(), a_expr);
        values.insert("b".to_string(), b_expr);

        let result = z3_il::solve_multi(&[constraint1, constraint2], &values)?.unwrap();

        assert_eq!(result["a"].value_u64().unwrap(), 3);
        assert_eq!(result["b"].value_u64().unwrap(), 7);

        Ok(())
    }

    #[test]
    fn test_unsat() -> Result<()> {
        // x == 5 AND x == 10 is unsatisfiable
        let constraint1 = il::Expression::cmpeq(il::expr_scalar("x", 32), il::expr_const(5, 32))?;
        let constraint2 = il::Expression::cmpeq(il::expr_scalar("x", 32), il::expr_const(10, 32))?;

        let result = z3_il::solve(&[constraint1, constraint2], &il::expr_scalar("x", 32))?;
        assert!(result.is_none());

        Ok(())
    }

    #[test]
    fn test_bitwise_ops() -> Result<()> {
        let config = Config::new()?.enable_model()?;
        let context = Context::new(config)?;
        let solver = Solver::new(&context)?;

        let sort8 = context.mk_bv_sort(8)?;
        let a = context.mk_numeral(0xAA, &sort8)?; // 10101010
        let b = context.mk_numeral(0x0F, &sort8)?; // 00001111
        let result_var = context.mk_var("r", &sort8)?;

        // Test bvand: 0xAA & 0x0F = 0x0A
        solver.assert(&context.eq(&result_var, &context.bvand(&a, &b)?)?);
        let model = Model::new(&context, &solver).unwrap();
        let val = model.get_const_interp(&result_var).unwrap();
        let val_str = val.get_numeral_decimal_string(&context).unwrap();
        assert_eq!(val_str.parse::<u64>().unwrap(), 0x0A);

        // Test bvor: 0xAA | 0x0F = 0xAF
        let solver2 = Solver::new(&context)?;
        solver2.assert(&context.eq(&result_var, &context.bvor(&a, &b)?)?);
        let model2 = Model::new(&context, &solver2).unwrap();
        let val2 = model2.get_const_interp(&result_var).unwrap();
        let val2_str = val2.get_numeral_decimal_string(&context).unwrap();
        assert_eq!(val2_str.parse::<u64>().unwrap(), 0xAF);

        // Test bvxor: 0xAA ^ 0x0F = 0xA5
        let solver3 = Solver::new(&context)?;
        solver3.assert(&context.eq(&result_var, &context.bvxor(&a, &b)?)?);
        let model3 = Model::new(&context, &solver3).unwrap();
        let val3 = model3.get_const_interp(&result_var).unwrap();
        let val3_str = val3.get_numeral_decimal_string(&context).unwrap();
        assert_eq!(val3_str.parse::<u64>().unwrap(), 0xA5);

        // Test bvnot: !0xAA = 0x55
        let solver4 = Solver::new(&context)?;
        solver4.assert(&context.eq(&result_var, &context.bvnot(&a)?)?);
        let model4 = Model::new(&context, &solver4).unwrap();
        let val4 = model4.get_const_interp(&result_var).unwrap();
        let val4_str = val4.get_numeral_decimal_string(&context).unwrap();
        assert_eq!(val4_str.parse::<u64>().unwrap(), 0x55);

        Ok(())
    }

    #[test]
    fn test_concat_extract_ext() -> Result<()> {
        let config = Config::new()?.enable_model()?;
        let context = Context::new(config)?;

        let sort8 = context.mk_bv_sort(8)?;
        let sort16 = context.mk_bv_sort(16)?;
        let hi = context.mk_numeral(0xAB, &sort8)?;
        let lo = context.mk_numeral(0xCD, &sort8)?;

        // concat(0xAB, 0xCD) = 0xABCD
        let solver = Solver::new(&context)?;
        let result_var = context.mk_var("r", &sort16)?;
        solver.assert(&context.eq(&result_var, &context.concat(&hi, &lo)?)?);
        let model = Model::new(&context, &solver).unwrap();
        let val = model.get_const_interp(&result_var).unwrap();
        let val_str = val.get_numeral_decimal_string(&context).unwrap();
        assert_eq!(val_str.parse::<u64>().unwrap(), 0xABCD);

        // extract bits [7:0] of 0xABCD = 0xCD
        let solver2 = Solver::new(&context)?;
        let result8 = context.mk_var("r8", &sort8)?;
        let concat_val = context.concat(&hi, &lo)?;
        solver2.assert(&context.eq(&result8, &context.extract(7, 0, &concat_val)?)?);
        let model2 = Model::new(&context, &solver2).unwrap();
        let val2 = model2.get_const_interp(&result8).unwrap();
        let val2_str = val2.get_numeral_decimal_string(&context).unwrap();
        assert_eq!(val2_str.parse::<u64>().unwrap(), 0xCD);

        // sign_ext 0x80 (8-bit) to 16-bit = 0xFF80
        let neg = context.mk_numeral(0x80, &sort8)?;
        let solver3 = Solver::new(&context)?;
        let result16 = context.mk_var("r16", &sort16)?;
        solver3.assert(&context.eq(&result16, &context.sign_ext(8, &neg)?)?);
        let model3 = Model::new(&context, &solver3).unwrap();
        let val3 = model3.get_const_interp(&result16).unwrap();
        let val3_str = val3.get_numeral_decimal_string(&context).unwrap();
        assert_eq!(val3_str.parse::<u64>().unwrap(), 0xFF80);

        // zero_ext 0x80 (8-bit) to 16-bit = 0x0080
        let solver4 = Solver::new(&context)?;
        let result16b = context.mk_var("r16b", &sort16)?;
        solver4.assert(&context.eq(&result16b, &context.zero_ext(8, &neg)?)?);
        let model4 = Model::new(&context, &solver4).unwrap();
        let val4 = model4.get_const_interp(&result16b).unwrap();
        let val4_str = val4.get_numeral_decimal_string(&context).unwrap();
        assert_eq!(val4_str.parse::<u64>().unwrap(), 0x0080);

        Ok(())
    }
}
