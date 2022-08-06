use ceceio::{
    parse_expression, Atom, Env, Evaluable, Expression,
};

fn main() {
    let expr = parse_expression("(if true 2)").unwrap().1;
    dbg!(&expr);
    assert_eq!(
        dbg!(expr.evaluate(&mut Env {}).unwrap()),
        Expression::Atom(Atom::Number(2.0))
    );

    let expr =
        parse_expression("(if false 2 (if true 5))").unwrap().1;

    assert_eq!(
        expr.evaluate(&mut Env {}).unwrap(),
        Expression::Atom(Atom::Number(5.0))
    );

    let expr = parse_expression("(+ 2 3 4 +)").unwrap().1;
    println!("{}", expr.evaluate(&mut Env {}).unwrap_err());
}
