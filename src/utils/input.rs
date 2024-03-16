use std::io::{ stdin, stdout, Write };
use std::str::FromStr;
use std::fmt::Debug;

pub fn input<T>(prompt: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{

    print!("{}", prompt);
    stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().parse().unwrap_or_else(|err| {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    })

}
