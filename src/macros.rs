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