use std::env;

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut num_buf = String::new();

    for c in input.chars() {
        if !num_buf.is_empty() && !c.is_ascii_digit() {
            tokens.push(Token::Num(num_buf.parse().unwrap()));
            num_buf.clear();
        }
        match c {
            ' ' | '\n' | '\t' => {},
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            '0'..='9' => {
                num_buf.push(c);
            },
            _ => panic!("Unknown Char: {}", c),
        }
    }

    if !num_buf.is_empty() {
        tokens.push(Token::Num(num_buf.parse().unwrap()));
        num_buf.clear();
    }
    tokens
}
#[derive(Debug, PartialEq)]
enum Token {
    Num(i64),
    Plus,
    Minus,
    Star,
    RParen,
    LParen,
    Slash,
}
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let filename = env::args().nth(1).ok_or("err")?;
    let contents = std::fs::read_to_string(filename)?;
    
    for line in contents.lines() {
        for t in tokenize(line){
            match t {
                Token::Num(n ) => println!("Number{}",n),
                Token::Plus => println!("Plus"),
                Token::Minus => println!("Minus"),
                Token::Star => println!("Star"),
                Token::RParen => println!("RParen"),
                Token::LParen => println!("LParen"),
                Token::Slash => println!("Slash"),
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_basic() {
        let input = "12 + 3 * (4 - 2)";
        let tokens = tokenize(input);

        let expected = vec![
            Token::Num(12),
            Token::Plus,
            Token::Num(3),
            Token::Star,
            Token::LParen,
            Token::Num(4),
            Token::Minus,
            Token::Num(2),
            Token::RParen,
        ];

        assert_eq!(tokens, expected);
    }
}