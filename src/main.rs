use ceceio::{parse_expression, Env, Evaluable};

fn main() {
    let expr =
        parse_expression("(/ (* 2 3) (- 5 6 7))").unwrap().1;

    match expr.evaluate(&mut Env {}) {
        Ok(evaluated) => println!("{evaluated}"),
        Err(err) => println!("Error: {err}"),
    }
}
