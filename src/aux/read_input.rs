use std::io;
use std::str::FromStr;

pub fn read_input<T>() -> Result<T, String>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| format!("Error reading input: {}", e))?;

    let input = buffer.trim();

    input
        .parse::<T>()
        .map_err(|e| format!("Error: {}", e))
}