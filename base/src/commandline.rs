use std::io;
use std::io::Write;

pub fn ask(question: &str) -> io::Result<String> {
    print!("{}", question);
    io::stdout().flush().unwrap();
    read_input()
}

pub fn askln(question: &str) -> io::Result<String> {
    println!("{}", question);
    read_input()
}

pub fn read_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input.split_off(input.len() - 1);
    Ok(input)
}
