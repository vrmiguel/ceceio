# ceceio

Embeddable tree-walk interpreter for a "mostly lazy" Lisp-like scripting language. Just a work-in-progress testbed for now.

## Sample usage

```rust
    use ceceio::Interpreter;

    let mut interp = Interpreter::new();

    assert!(interp
        .parse_and_eval(
            "(def fibonacci 
                (fn [n] 
                    (cond 
                        (= n 0) 1
                        (= n 1) 1
                        (+ (fibonacci (- n 1)) (fibonacci (- n 2))))))",
        )
        .is_ok());
    assert_eq!(
        interp.parse_and_eval("(fibonacci 5)").unwrap(),
        8.0.into()
    );

    assert!(
        interp.parse_and_eval("(def one-to-five '(1 2 3 4 5))").is_ok()
    );

    assert_eq!(
        interp
            .parse_and_eval(
                "(count (fn [x] (= (% x 2) 1)) one-to-five)"
            )
            .unwrap(),
        3.0.into()
    );

    assert_eq!(
        interp.parse_and_eval("(/ (* 2 3) (- 5 6 7))").unwrap(),
        (-0.75).into()
    );

    assert_eq!(
        interp.parse_and_eval("(def x 5)").unwrap(),
        5.0.into()
    );

    assert!(
        interp.parse_and_eval("(def twice (fn [x] (+ x x)))").is_ok()
    );

    assert_eq!(
        interp.parse_and_eval("(= (twice x) (* x 2) 10.0)").unwrap(),
        true.into()
    );

    assert_eq!(
        interp
            .parse_and_eval("(= (* x x x x x) 3125.0)")
            .unwrap(),
        true.into()
    );

    assert!(interp
        .parse_and_eval("(def id (fn [x] x))")
        .is_ok());
    assert_eq!(
        interp.parse_and_eval("(= (id 2) 2)").unwrap(),
        true.into()
    );
    assert_eq!(
        interp
            .parse_and_eval("(= (id :success) :success)")
            .unwrap(),
        true.into()
    );

    assert!(interp
        .parse_and_eval("(def double (fn [x] (+ x x)))")
        .is_ok());
    assert_eq!(
        interp.parse_and_eval("(= (double 2) (* 4 1))").unwrap(),
        true.into()
    );

    assert!(interp
        .parse_and_eval("(def even? (fn [x] (= (% x 2) 0)))")
        .is_ok());
    assert_eq!(
        interp.parse_and_eval("(even? 2)").unwrap(),
        true.into()
    );
    assert_eq!(
        interp.parse_and_eval("(even? 3)").unwrap(),
        false.into()
    );
```
