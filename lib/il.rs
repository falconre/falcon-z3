use Ast;
use Check;
use Config;
use Context;
use Solver;
use Model;
use error::*;
use falcon::il;

pub enum SolverResult {
    Unsat,
    Unknown,
    Sat(il::Constant)
}


pub fn solve(constraints: &[il::Expression], value: &il::Expression)
    -> Result<Option<il::Constant>> {

    let config = Config::new().enable_model();
    let context = Context::new(config);
    let solver = Solver::new(&context);

    let sort = context.mk_bv_sort(1);
    let one = context.mk_numeral(1, &sort)?;

    for constraint in constraints {
        solver.assert(&context.eq(&one,
                                  &expression_to_ast(&context, constraint)?));
    }

    Ok(match solver.check() {
        Check::Unsat |
        Check::Unknown => None,
        Check::Sat => {
            let ast_value = expression_to_ast(&context, value)?;
            Model::new(&context, &solver)
                .and_then(|model| model.get_const_interp(&ast_value))
                .and_then(|constant_ast|
                        constant_ast.get_numeral_decimal_string(&context))
                .and_then(|numeral_dec_str| {
                    il::Constant::from_decimal_string(
                        &numeral_dec_str,
                        value.bits()
                    ).ok()
                })
        }
    })
}


pub fn expression_to_ast(context: &Context, expression: &il::Expression)
    -> Result<Ast> {
    Ok(match *expression {
        il::Expression::Scalar(ref scalar) => {
            let sort = context.mk_bv_sort(scalar.bits());
            context.mk_var(scalar.name(), &sort)?
        },
        il::Expression::Constant(ref constant) => {
            if constant.bits() <= 64 {
                let sort = context.mk_bv_sort(constant.bits());
                context.mk_numeral(constant.value_u64().unwrap(), &sort)?
            }
            else {
                let big_uint = constant.value_big().unwrap();
                let sort = context.mk_bv_sort(constant.bits());
                let mut v = context.mk_numeral(big_uint.to_bytes_be()[0] as u64,
                                               &sort)?;
                for _ in 1..(constant.bits() / 8) {
                    v = context.concat(
                        &v,
                        &context.mk_numeral(big_uint.to_bytes_be()[0] as u64,
                                            &sort)?);
                }
                v
            }
        },
        il::Expression::Add(ref lhs, ref rhs) =>
            context.bvadd(&expression_to_ast(context, lhs)?,
                          &expression_to_ast(context, rhs)?),
        il::Expression::Sub(ref lhs, ref rhs) =>
            context.bvsub(&expression_to_ast(context, lhs)?,
                          &expression_to_ast(context, rhs)?),
        il::Expression::Mul(ref lhs, ref rhs) =>
            context.bvmul(&expression_to_ast(context, lhs)?,
                          &expression_to_ast(context, rhs)?),
        il::Expression::Divu(ref lhs, ref rhs) =>
            context.bvudiv(&expression_to_ast(context, lhs)?,
                           &expression_to_ast(context, rhs)?),
        il::Expression::Modu(ref lhs, ref rhs) =>
            context.bvurem(&expression_to_ast(context, lhs)?,
                           &expression_to_ast(context, rhs)?),
        il::Expression::Divs(ref lhs, ref rhs) =>
            context.bvsdiv(&expression_to_ast(context, lhs)?,
                           &expression_to_ast(context, rhs)?),
        il::Expression::Mods(ref lhs, ref rhs) =>
            context.bvsrem(&expression_to_ast(context, lhs)?,
                           &expression_to_ast(context, rhs)?),
        il::Expression::And(ref lhs, ref rhs) =>
            context.bvand(&expression_to_ast(context, lhs)?,
                          &expression_to_ast(context, rhs)?),
        il::Expression::Or(ref lhs, ref rhs) =>
            context.bvor(&expression_to_ast(context, lhs)?,
                         &expression_to_ast(context, rhs)?),
        il::Expression::Xor(ref lhs, ref rhs) =>
            context.bvxor(&expression_to_ast(context, lhs)?,
                         &expression_to_ast(context, rhs)?),
        il::Expression::Shl(ref lhs, ref rhs) =>
            context.bvshl(&expression_to_ast(context, lhs)?,
                          &expression_to_ast(context, rhs)?),
        il::Expression::Shr(ref lhs, ref rhs) =>
            context.bvlshr(&expression_to_ast(context, lhs)?,
                          &expression_to_ast(context, rhs)?),
        il::Expression::Cmpeq(ref lhs, ref rhs) => {
            let sort = context.mk_bv_sort(1);
            context.ite(&context.eq(&expression_to_ast(context, lhs)?,
                                    &expression_to_ast(context, rhs)?),
                        &context.mk_numeral(1, &sort)?,
                        &context.mk_numeral(0, &sort)?)
        },
        il::Expression::Cmpneq(ref lhs, ref rhs) => {
            let sort = context.mk_bv_sort(1);
            context.ite(&context.eq(&expression_to_ast(context, lhs)?,
                                    &expression_to_ast(context, rhs)?),
                        &context.mk_numeral(0, &sort)?,
                        &context.mk_numeral(1, &sort)?)
        },
        il::Expression::Cmplts(ref lhs, ref rhs) => {
            let sort = context.mk_bv_sort(1);
            context.ite(&context.bvslt(&expression_to_ast(context, lhs)?,
                                       &expression_to_ast(context, rhs)?),
                        &context.mk_numeral(1, &sort)?,
                        &context.mk_numeral(0, &sort)?)
        },
        il::Expression::Cmpltu(ref lhs, ref rhs) => {
            let sort = context.mk_bv_sort(1);
            context.ite(&context.bvult(&expression_to_ast(context, lhs)?,
                                       &expression_to_ast(context, rhs)?),
                        &context.mk_numeral(1, &sort)?,
                        &context.mk_numeral(0, &sort)?)
        },
        il::Expression::Zext(bits, ref rhs) =>
            context.zero_ext((bits - rhs.bits()) as u32,
                             &expression_to_ast(context, rhs)?),
        il::Expression::Sext(bits, ref rhs) =>
            context.sign_ext((bits - rhs.bits()) as u32,
                             &expression_to_ast(context, rhs)?),
        il::Expression::Trun(bits, ref rhs) => {
            context.extract((bits - 1) as u32, 0,
                            &expression_to_ast(context, rhs)?)
        }
    })
}