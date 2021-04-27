use error::*;
use falcon::il;
use std::collections::HashMap;
use Ast;
use Check;
use Config;
use Context;
use Model;
use Optimize;
use Solver;

pub enum SolverResult {
    Unsat,
    Unknown,
    Sat(il::Constant),
}

fn return_solver_result(
    solver: &Solver,
    context: &Context,
    ast: &Ast,
    bits: usize,
) -> Option<il::Constant> {
    match solver.check() {
        Check::Unsat | Check::Unknown => None,
        Check::Sat => Model::new(&context, &solver)
            .and_then(|model| model.get_const_interp(&ast))
            .and_then(|constant_ast| constant_ast.get_numeral_decimal_string(&context))
            .and_then(|numeral_dec_str| {
                il::Constant::from_decimal_string(&numeral_dec_str, bits).ok()
            }),
    }
}

fn return_optimize_result(
    optimize: &Optimize,
    context: &Context,
    ast: &Ast,
    bits: usize,
) -> Option<il::Constant> {
    match optimize.check() {
        Check::Unsat | Check::Unknown => None,
        Check::Sat => Model::new_optimize(&context, &optimize)
            .and_then(|model| model.get_const_interp(&ast))
            .and_then(|constant_ast| constant_ast.get_numeral_decimal_string(&context))
            .and_then(|numeral_dec_str| {
                il::Constant::from_decimal_string(&numeral_dec_str, bits).ok()
            }),
    }
}

fn solver_init(solver: &Solver, context: &Context, constraints: &[il::Expression]) -> Result<()> {
    let sort = context.mk_bv_sort(1);
    let one = context.mk_numeral(1, &sort)?;

    for constraint in constraints {
        solver.assert(&context.eq(&one, &expression_to_ast(&context, constraint)?));
    }

    Ok(())
}

fn optimize_init(
    optimize: &Optimize,
    context: &Context,
    constraints: &[il::Expression],
) -> Result<()> {
    let sort = context.mk_bv_sort(1);
    let one = context.mk_numeral(1, &sort)?;

    for constraint in constraints {
        optimize.assert(&context.eq(&one, &expression_to_ast(&context, constraint)?));
    }

    Ok(())
}

pub fn maximize(
    constraints: &[il::Expression],
    value: &il::Expression,
) -> Result<Option<il::Constant>> {
    let config = Config::new().enable_model();
    let context = Context::new(config);
    let optimize = Optimize::new(&context);

    optimize_init(&optimize, &context, constraints)?;

    let optimize_result = context.mk_var("OPTIMIZE_RESULT", &context.mk_bv_sort(value.bits()))?;

    optimize.assert(&context.eq(&optimize_result, &expression_to_ast(&context, value)?));

    optimize.maximize(&optimize_result);

    Ok(return_optimize_result(
        &optimize,
        &context,
        &optimize_result,
        value.bits(),
    ))
}

pub fn minimize(
    constraints: &[il::Expression],
    value: &il::Expression,
) -> Result<Option<il::Constant>> {
    let config = Config::new().enable_model();
    let context = Context::new(config);
    let optimize = Optimize::new(&context);

    optimize_init(&optimize, &context, constraints)?;

    let optimize_result = context.mk_var("OPTIMIZE_RESULT", &context.mk_bv_sort(value.bits()))?;

    optimize.assert(&context.eq(&optimize_result, &expression_to_ast(&context, value)?));

    optimize.minimize(&optimize_result);

    Ok(return_optimize_result(
        &optimize,
        &context,
        &optimize_result,
        value.bits(),
    ))
}

pub fn solve(
    constraints: &[il::Expression],
    value: &il::Expression,
) -> Result<Option<il::Constant>> {
    let config = Config::new().enable_model();
    let context = Context::new(config);
    let solver = Solver::new(&context);

    solver_init(&solver, &context, constraints)?;

    let solver_result = context.mk_var("SOLVER_RESULT", &context.mk_bv_sort(value.bits()))?;

    solver.assert(&context.eq(&solver_result, &expression_to_ast(&context, value)?));

    Ok(return_solver_result(
        &solver,
        &context,
        &solver_result,
        value.bits(),
    ))
}

pub fn solve_multi(
    constraints: &[il::Expression],
    values: &HashMap<String, il::Expression>,
) -> Result<Option<HashMap<String, il::Constant>>> {
    let config = Config::new().enable_model();
    let context = Context::new(config);
    let solver = Solver::new(&context);

    let sort = context.mk_bv_sort(1);
    let one = context.mk_numeral(1, &sort)?;

    for constraint in constraints {
        solver.assert(&context.eq(&one, &expression_to_ast(&context, constraint)?));
    }

    let mut solver_variables = HashMap::new();

    for (ref name, ref expression) in values {
        let var = context.mk_var(name.to_string(), &context.mk_bv_sort(expression.bits()))?;
        solver.assert(&context.eq(&var, &expression_to_ast(&context, &expression)?));
        solver_variables.insert(name.to_string(), var);
    }

    Ok(match solver.check() {
        Check::Unsat | Check::Unknown => None,
        Check::Sat => {
            //let ast_value = expression_to_ast(&context, value)?;
            Model::new(&context, &solver).map(|model| {
                values
                    .iter()
                    .map(|(name, expr)| {
                        let var = &solver_variables[name];
                        let constant_ast = model.get_const_interp(&var).unwrap();
                        let dec_str = constant_ast.get_numeral_decimal_string(&context).unwrap();
                        let constant =
                            il::Constant::from_decimal_string(&dec_str, expr.bits()).unwrap();
                        (name.to_string(), constant)
                    })
                    .collect::<HashMap<String, il::Constant>>()
            })
        }
    })
}

pub fn expression_to_ast(context: &Context, expression: &il::Expression) -> Result<Ast> {
    Ok(match *expression {
        il::Expression::Scalar(ref scalar) => {
            let sort = context.mk_bv_sort(scalar.bits());
            context.mk_var(scalar.name(), &sort)?
        }
        il::Expression::Constant(ref constant) => {
            if let Some(value) = constant.value_u64() {
                let sort = context.mk_bv_sort(constant.bits());
                context.mk_numeral(value, &sort)?
            } else {
                let big_uint = constant.value();
                let sort = context.mk_bv_sort(8);
                let bytes = big_uint.to_bytes_le();
                let mut v = if bytes.is_empty() {
                    context.mk_numeral(0, &sort)?
                } else {
                    context.mk_numeral(bytes[0] as u64, &sort)?
                };
                for i in 1..(constant.bits() / 8) {
                    let numeral = if bytes.len() <= i {
                        context.mk_numeral(0, &sort)?
                    } else {
                        context.mk_numeral(bytes[i] as u64, &sort)?
                    };
                    v = context.concat(&numeral, &v);
                }
                v
            }
        }
        il::Expression::Add(ref lhs, ref rhs) => context.bvadd(
            &expression_to_ast(context, lhs)?,
            &expression_to_ast(context, rhs)?,
        ),
        il::Expression::Sub(ref lhs, ref rhs) => context.bvsub(
            &expression_to_ast(context, lhs)?,
            &expression_to_ast(context, rhs)?,
        ),
        il::Expression::Mul(ref lhs, ref rhs) => context.bvmul(
            &expression_to_ast(context, lhs)?,
            &expression_to_ast(context, rhs)?,
        ),
        il::Expression::Divu(ref lhs, ref rhs) => context.bvudiv(
            &expression_to_ast(context, lhs)?,
            &expression_to_ast(context, rhs)?,
        ),
        il::Expression::Modu(ref lhs, ref rhs) => context.bvurem(
            &expression_to_ast(context, lhs)?,
            &expression_to_ast(context, rhs)?,
        ),
        il::Expression::Divs(ref lhs, ref rhs) => context.bvsdiv(
            &expression_to_ast(context, lhs)?,
            &expression_to_ast(context, rhs)?,
        ),
        il::Expression::Mods(ref lhs, ref rhs) => context.bvsrem(
            &expression_to_ast(context, lhs)?,
            &expression_to_ast(context, rhs)?,
        ),
        il::Expression::And(ref lhs, ref rhs) => context.bvand(
            &expression_to_ast(context, lhs)?,
            &expression_to_ast(context, rhs)?,
        ),
        il::Expression::Or(ref lhs, ref rhs) => context.bvor(
            &expression_to_ast(context, lhs)?,
            &expression_to_ast(context, rhs)?,
        ),
        il::Expression::Xor(ref lhs, ref rhs) => context.bvxor(
            &expression_to_ast(context, lhs)?,
            &expression_to_ast(context, rhs)?,
        ),
        il::Expression::Shl(ref lhs, ref rhs) => context.bvshl(
            &expression_to_ast(context, lhs)?,
            &expression_to_ast(context, rhs)?,
        ),
        il::Expression::Shr(ref lhs, ref rhs) => context.bvlshr(
            &expression_to_ast(context, lhs)?,
            &expression_to_ast(context, rhs)?,
        ),
        il::Expression::Cmpeq(ref lhs, ref rhs) => {
            let sort = context.mk_bv_sort(1);
            context.ite(
                &context.eq(
                    &expression_to_ast(context, lhs)?,
                    &expression_to_ast(context, rhs)?,
                ),
                &context.mk_numeral(1, &sort)?,
                &context.mk_numeral(0, &sort)?,
            )
        }
        il::Expression::Cmpneq(ref lhs, ref rhs) => {
            let sort = context.mk_bv_sort(1);
            context.ite(
                &context.eq(
                    &expression_to_ast(context, lhs)?,
                    &expression_to_ast(context, rhs)?,
                ),
                &context.mk_numeral(0, &sort)?,
                &context.mk_numeral(1, &sort)?,
            )
        }
        il::Expression::Cmplts(ref lhs, ref rhs) => {
            let sort = context.mk_bv_sort(1);
            context.ite(
                &context.bvslt(
                    &expression_to_ast(context, lhs)?,
                    &expression_to_ast(context, rhs)?,
                ),
                &context.mk_numeral(1, &sort)?,
                &context.mk_numeral(0, &sort)?,
            )
        }
        il::Expression::Cmpltu(ref lhs, ref rhs) => {
            let sort = context.mk_bv_sort(1);
            context.ite(
                &context.bvult(
                    &expression_to_ast(context, lhs)?,
                    &expression_to_ast(context, rhs)?,
                ),
                &context.mk_numeral(1, &sort)?,
                &context.mk_numeral(0, &sort)?,
            )
        }
        il::Expression::Zext(bits, ref rhs) => context.zero_ext(
            (bits - rhs.bits()) as u32,
            &expression_to_ast(context, rhs)?,
        ),
        il::Expression::Sext(bits, ref rhs) => context.sign_ext(
            (bits - rhs.bits()) as u32,
            &expression_to_ast(context, rhs)?,
        ),
        il::Expression::Trun(bits, ref rhs) => {
            context.extract((bits - 1) as u32, 0, &expression_to_ast(context, rhs)?)
        }
        il::Expression::Ite(ref cond, ref then, ref else_) => context.ite(
            &context.eq(
                &expression_to_ast(context, cond)?,
                &context.mk_numeral(1, &context.mk_bv_sort(1))?,
            ),
            &expression_to_ast(context, then)?,
            &expression_to_ast(context, else_)?,
        ),
    })
}

#[test]
fn test_solve() {
    let expression = il::expr_const(32, 32);
    assert_eq!(
        solve(&[], &expression).unwrap().unwrap(),
        il::const_(32, 32)
    );

    let expression = il::expr_const(0x1000, 32);
    assert_eq!(
        solve(&[], &expression).unwrap().unwrap(),
        il::const_(0x1000, 32)
    );

    let expression = il::expr_const(0x12345678, 32);
    assert_eq!(
        solve(&[], &expression).unwrap().unwrap(),
        il::const_(0x12345678, 32)
    );

    let expression = il::Expression::add(il::expr_const(32, 32), il::expr_const(1, 32)).unwrap();
    assert_eq!(
        solve(&[], &expression).unwrap().unwrap(),
        il::const_(33, 32)
    );

    let expression =
        il::Expression::add(il::expr_const(0x420000, 32), il::expr_const(0xffffc000, 32)).unwrap();
    assert_eq!(
        solve(&[], &expression).unwrap().unwrap(),
        il::const_(0x41c000, 32)
    );

    let expression = il::Expression::add(
        il::Expression::add(il::expr_const(0x420000, 32), il::expr_const(0xffffc000, 32)).unwrap(),
        il::expr_const(0xffff83dc, 32),
    )
    .unwrap();
    assert_eq!(
        solve(&[], &expression).unwrap().unwrap(),
        il::const_(0x4143dc, 32)
    );
}

#[test]
fn test_maximize() -> Result<()> {
    let rdx = il::expr_scalar("rdx", 64);

    let constraint = il::Expression::cmpltu(rdx.clone(), il::expr_const(27, 64))?;

    let value = maximize(&[constraint], &rdx)?.unwrap();

    assert_eq!(value.value_u64().unwrap(), 26);

    Ok(())
}

#[test]
fn test_minimize() -> Result<()> {
    let rdx = il::expr_scalar("rdx", 64);

    let constraint = il::Expression::cmpltu(il::expr_const(27, 64), rdx.clone())?;

    let value = minimize(&[constraint], &rdx)?.unwrap();

    assert_eq!(value.value_u64().unwrap(), 28);

    Ok(())
}
