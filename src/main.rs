use std::io::Read;

use lexer::Lexer;

fn main() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin
        .lock()
        .read_to_string(&mut input)
        .expect("Read to string failed.");

    println!("{:?}", input);

    let mut lexer = Lexer::new(&input);
    let result = lexer.lex();

    println!("{:?}", result);
}
