use std::error::Error;

use mrbasm::lexer;

fn main() -> Result<(), Box<dyn Error>> {
    let source = include_str!("masm/test_1.mas");
    let mut l = lexer::Lexer::new(source);
    let _ = l.scan()?;
    dbg!(l.tokens);
    Ok(())
}
