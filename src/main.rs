use std::{
    fs::File,
    io::{self, BufReader},
    path::Path,
};

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

    assert!(interp
        .parse_and_eval("(def ok (fn [] :ok))")
        .is_ok());

    assert_eq!(
        interp.parse_and_eval("(= (ok) :ok)").unwrap(),
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

    assert!(interp
        .parse_and_eval(
            "(def double-if-even (fn [x] (if (even? x) (* x 2))))"
        )
        .is_ok());

    assert_eq!(
        interp.parse_and_eval("(double-if-even 4)").unwrap(),
        8.0.into()
    );

    let file = argv::iter().nth(1).unwrap();

    let mut reader = ReallocBufReader::from(file).unwrap();

    while let Some(line) = reader.read_line().unwrap() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        interp.parse_and_eval(line).unwrap();
    }
}

struct ReallocBufReader {
    reader: BufReader<File>,
    buffer: String,
}

impl ReallocBufReader {
    pub fn from<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let buffer = String::with_capacity(1024);

        Ok(Self { reader, buffer })
    }

    pub fn read_line(&mut self) -> io::Result<Option<&str>> {
        use std::io::BufRead;

        self.buffer.clear();

        let bytes_read =
            self.reader.read_line(&mut self.buffer)?;

        Ok((bytes_read != 0).then_some(self.buffer.as_str()))
    }
}
