use std::io;
use std::io::Write;

///
///  a function to ask an inline question to the user
///  if successful returning Ok containing the answer
///
pub fn ask(question: &str) -> io::Result<String> {
    print!("{}", question);
    io::stdout().flush().unwrap();
    read_input()
}

///
/// the same as ask but with a new line between the question and the user input
///
pub fn askln(question: &str) -> io::Result<String> {
    println!("{}", question);
    read_input()
}

///
/// read in a line discarding the line break
///
pub fn read_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input.split_off(input.len() - 1);
    Ok(input)
}
