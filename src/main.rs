use ceceio::{Atom, Expression, Interpreter};

fn main() {
    let mut interp = Interpreter::new();

    assert_eq!(
        interp.parse_and_eval("(/ (* 2 3) (- 5 6 7))").unwrap(),
        Expression::Atom(Atom::Number(-0.75))
    );

    assert_eq!(
        interp.parse_and_eval("(def x 5)").unwrap(),
        Expression::Atom(Atom::Number(5.0))
    );

    assert_eq!(
        interp.parse_and_eval("(def twice (+ x x))").unwrap(),
        Expression::Atom(Atom::Number(10.0))
    );

    assert_eq!(
        interp.parse_and_eval("(= twice (* x 2) 10.0)").unwrap(),
        Expression::Atom(Atom::Boolean(true))
    );
}
