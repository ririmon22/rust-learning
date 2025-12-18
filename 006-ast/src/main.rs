use std::env;

struct Parser{
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0}
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    
    fn consume(&mut self) -> Token {
        let tok = self.tokens[self.pos].clone();
        self.pos  += 1;
        tok
    }
    
    fn parse(&mut self) -> Expr {
        let expr = self.parse_expr();
        if self.pos != self.tokens.len() {
            panic!("unexpected token after expression");
        }
        expr
    }
    
    fn parse_factor(&mut self) -> Expr {
        let t = self.peek().unwrap().clone();
        match t {
            Token::Num(n) => {
                self.consume();
                Expr::Number(n)
            },
            Token::LParen => {
                self.consume();
                let expr = self.parse_expr();
                let token = self.peek().unwrap().clone();
                match token{
                    Token::RParen => {
                        self.consume();
                    },
                    _ => panic!("Not RParen"),
                }
                expr
            },
            _ => panic!("Not Number"),
        }
    }
    
    fn parse_expr(&mut self) -> Expr {
      let mut left = self.parse_term();
      
      loop {
        let op = match self.peek() {
            Some(Token::Plus) => Operator::Plus,
            Some(Token::Minus) => Operator::Minus,
            _ => break,
        };
        
        self.consume();
        let right = self.parse_term();

        left = Expr::Binary { left: Box::new(left), op, right: Box::new(right), };
      }
      left
    }
    
    fn parse_term(&mut self) -> Expr {
      let mut left = self.parse_factor();
      
      loop {
        let op = match self.peek() {
            Some(Token::Star) => Operator::Star,
            Some(Token::Slash) => Operator::Slash,
            _ => break,
        };
        
        self.consume();
        let right = self.parse_factor();

        left = Expr::Binary { left: Box::new(left), op, right: Box::new(right), };
      }
      left
    }

}
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
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Num(i64),
    Plus,
    Minus,
    Star,
    RParen,
    LParen,
    Slash,
}
#[derive(Debug, Clone)]
enum Expr {
    Number(i64),
    Binary {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    }
}

#[derive(Debug, Clone)]
enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
}

fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Binary{left, op, right} => {
            match op {
                Operator::Plus => eval(left) + eval(right),
                Operator::Minus => eval(left) - eval(right),
                Operator::Star => eval(left) * eval(right),
                Operator::Slash => eval(left) / eval(right),
            }
        },
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = env::args().nth(1).ok_or("err")?;
    let contents = std::fs::read_to_string(filename)?;

    for line in contents.lines() {
        let mut parser = Parser::new(tokenize(line));
        let expr = parser.parse();
        println!("{}",eval(&expr));
    }
    Ok(())
}

#[cfg(test)]
mod test{

    use super::*;
    #[test]
    fn eval_number_test() {
        let expr = Expr::Number(42);
        assert_eq!(eval(&expr), 42);
    }
    
    #[test]
    fn eval_binary_plus_test() {
        let expr = Expr::Binary { left: Box::new(Expr::Number(1)), op: Operator::Plus, right: Box::new(Expr::Number(2)), };
        assert_eq!(eval(&expr), 3);
    }
    #[test]
    fn eval_nested() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Number(1)),
            op: Operator::Plus,
            right: Box::new(
                Expr::Binary {
                    left: Box::new(Expr::Number(2)),
                    op: Operator::Star,
                    right: Box::new(Expr::Number(3)),
                }
            ),
        };

        assert_eq!(eval(&expr), 7);
    }
    
    #[test]
    fn test_main() {
        let input = "10 - 4 / 2";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);

        let expr = parser.parse();
        let result = eval(&expr);
        assert_eq!(result, 8);
    }
}