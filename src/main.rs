#[derive(Debug, Clone)]
enum Token {
    Number(i64),
    Plus,
    Dash,
    Star,
    Slash,
    LeftParen,
    RightParen,
    EOF,
}

#[derive(Debug)]
struct SyntaxError {
    message: String,
}

impl SyntaxError { 
    fn new(message: String) -> Self {
        SyntaxError {
            message,
        }
    }
}

fn tokenizer(input: String) -> Result<Vec<Token>, SyntaxError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars().peekable(); // Peekable iterator to look ahead without consuming

    while let Some(&ch) = chars.peek() {
        match ch {
            ch if ch.is_whitespace() => {
                chars.next(); 
            }
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next(); 
            }
            '-' => {
                tokens.push(Token::Dash);
                chars.next(); 
            }
            '*' => {
                tokens.push(Token::Star);
                chars.next(); 
            }
            '/' => {
                tokens.push(Token::Slash);
                chars.next(); 
            }
            '0'..='9' => {
                let mut number = String::new();
                while let Some(&digit) = chars.peek() {
                    if digit.is_ascii_digit() {
                        number.push(digit);
                        chars.next(); // Advance the iterator manually for each digit
                    } else {
                        break;
                    }
                }
                let value: i64 = number.parse::<i64>().unwrap();
                tokens.push(Token::Number(value));
            }
            _ => {
                return Err(SyntaxError::new(format!("unrecognized character {}", ch)));
            }
        }
    }

    tokens.push(Token::EOF);
    Ok(tokens)
}


//Box is a smart pointer, it allocated data on the heap who's size is not known at compile time
//the Box pointer itself is stored on the sack
#[allow(dead_code)]
#[derive(Debug)]
enum ASTNode {
    Number(i64),
    BinaryOp {
        op: Token,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        // -> Self means return an instance of same type
        Parser { tokens, current: 0 }
    }

    //Check current Token Without Advancing
    fn peek(&self) -> Option<&Token> {
        let token = self.tokens.get(self.current);
        println!("Peeking: {:?} -> {:?}", self.current, token);
        token
    }
    //Go to next token, add one to current index
    fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.current).cloned();
        println!("Advancing: {:?} -> {:?}", self.current, token);
        self.current += 1;
        token
    }

    //IF number return ASTNode with the number if keft paren parse the subexpression and check for right paren
    fn parse_factor(&mut self) -> Result<ASTNode, SyntaxError> {
        match self.advance() {
            Some(Token::Number(value)) => {
                println!("Parsed Number: {}", value);
                Ok(ASTNode::Number(value)) // Return a Number node
            }
            Some(Token::LeftParen) => {
                println!("Parsing subexpression inside parentheses");
                // Recursively parse the subexpression
                let node = self.parse_expression()?; // Parse the subexpression
                
                // Check for matching RightParen after parsing the subexpression
                if let Some(Token::RightParen) = self.peek() {
                    self.advance(); // Consume the RightParen
                    println!("Matched closing parenthesis");
                    Ok(node) // Successfully parsed and matched parentheses
                } else {
                    Err(SyntaxError::new(format!(
                        "Expected ')', but found {:?}",
                        self.peek()
                    )))
                }
            }
            Some(token) => Err(SyntaxError::new(format!("Unexpected token: {:?}", token))),
            None => Err(SyntaxError::new("Unexpected end of input".to_string())),
        }
    }
    
    
    
    
    
    

    fn parse_term(&mut self) -> Result<ASTNode, SyntaxError> {
        let mut node = self.parse_factor()?;

        while let Some(token) = self.peek() {
            match token {
                Token::Star | Token::Slash => {
                    let op = self.advance().expect("Expected operator but found unexpected EOF");
                    let right = self.parse_factor()?;
                    node = ASTNode::BinaryOp { op, left: Box::new(node), right: Box::new(right) };
                }
                _ => {
                    break;
                }
            }
        }
        Ok(node)
    }

    fn parse_expression(&mut self) -> Result<ASTNode, SyntaxError> {
        let mut node = self.parse_term()?;
        while let Some(token) = self.peek() {
            match token {
                Token::Plus | Token::Dash => {
                    let op = self.advance().expect("Expected operator but found unexpected EOF");
                    let right = self.parse_term()?;
                    node = ASTNode::BinaryOp {          
                        op,
                        left: Box::new(node),
                        right: Box::new(right),
                    };
                }
                _ => break, // Stop processing if no matching operator is found
            }
        }
        Ok(node)
    }
    
    

    fn parse(&mut self) -> Result<ASTNode, SyntaxError> {
        self.parse_expression()
    }
}

fn main() {
    let input = "3 + 5 * (((10 - 2)))".to_string();
    match tokenizer(input) {
        Ok(tokens) => {
            println!("Tokens: {:?}", tokens);
            let mut parser = Parser::new(tokens);
            match parser.parse() {
                Ok(ast) => println!("AST: {:#?}", ast),
                Err(err) => eprintln!("Parse Error: {}", err.message),
            }
        }
        Err(err) => eprintln!("Tokenization Error: {}", err.message),
    }
}
