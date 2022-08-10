use ceceio::Interpreter;

fn main() {
    let mut interp = Interpreter::new();

    assert_eq!(
        interp.parse_and_eval("(/ (* 2 3) (- 5 6 7))").unwrap(),
        (-0.75).into()
    );

    assert_eq!(
        interp.parse_and_eval("(def x 5)").unwrap(),
        5.0.into()
    );

    assert_eq!(
        interp.parse_and_eval("(def twice (+ x x))").unwrap(),
        10.0.into()
    );

    assert_eq!(
        interp.parse_and_eval("(= twice (* x 2) 10.0)").unwrap(),
        true.into()
    );

    assert_eq!(
        interp
            .parse_and_eval("(= (* x x x x x) 3125.0)")
            .unwrap(),
        true.into()
    );

    let arg = std::env::args().skip(1).next().unwrap();

    match interp.parse_and_eval(&arg) {
        Ok(expr) => println!("{expr}"),
        Err(err) => println!("{err}"),
    }
}
