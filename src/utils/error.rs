use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn print_error(error: &str) -> () {

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    
    stdout
        .set_color(ColorSpec::new()
        .set_fg(Some(Color::Red))).expect("Failed to set color");

    writeln!(&mut stdout, "{error}").expect("Failed to write to stdout");
    
    stdout
        .set_color(ColorSpec::new().set_fg(None))
        .expect("Failed to reset color");

}
