use std::error::Error;
use std::fmt::{Debug, Display};
use std::io::{self, Write};
use std::str::FromStr;

pub fn take_input<T>(say: T) -> Result<String, io::Error>
    where T: Display,
{
    print!("{say} ");
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

pub fn take_input_parse<T, N>(say: T) -> Result<N, Box<dyn Error>>
    where T: Display, N: FromStr, <N as FromStr>::Err: Debug
{
    let mut buffer = String::new();
    let mut parsed: Result<N, <N as FromStr>::Err>;
    loop {
        print!("{say} ");
        io::stdout().flush()?;
        buffer.clear();
        io::stdin().read_line(&mut buffer)?;
        parsed = buffer.trim().parse();
        if parsed.is_ok() {break}
    };
    Ok(parsed.unwrap())
}
