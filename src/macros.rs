#[macro_export]
macro_rules! pass {
    ($line:expr => $args:expr => $pass:expr) => ({
        use term_painter::ToStyle;
        use term_painter::Color::*;

        let args = $args.into();

        println!("{}\n\n{}", $line, Cyan.bold().paint(format!("input: {:?}\n", args)));

        let result = $pass(args)?;

        println!("{}\n", Cyan.bold().paint(format!("output: {:?}", result)));

        result
    })
}

/// This is a _really_ lazy parser that strips all whitespace.
#[cfg(test)]
#[macro_export]
macro_rules! assert_eq_no_ws {
    ($expected:expr, $actual:expr) => ({
        use std::str;

        fn strip_ws(input: &str) -> String {
            let mut result = String::new();

            for c in input.chars() {
                match c {
                    ' '|'\t'|'\n'|'\r' => (),
                    _ => result.push(c)
                }
            }

            result
        }

        let expected_str = str::from_utf8($expected).unwrap();
        let actual_str = str::from_utf8($actual).unwrap();

        let expected = strip_ws(expected_str);
        let actual = strip_ws(actual_str);

        if expected != actual {
            panic!("whitespace ignored values are not equal. expected: `{}` got: `{}`", expected_str, actual_str);
        }
    })
}
