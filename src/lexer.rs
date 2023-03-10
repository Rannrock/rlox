use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    // current position in input (points to current char)
    read_position: usize,
    // current reading position in input (after current char)
    ch: char, // current char under examination
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input,
            position: 0,
            read_position: 0,
            ch: '0',
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    pub fn next_token(&mut self) -> Token {
        let new_token: Token;
        match self.ch {
            '=' => {
                new_token = Token::EQUAL(self.ch);
            }
            ';' => {
                new_token = Token::SEMICOLON(self.ch);
            }
            '(' => {
                new_token = Token::LPAREN(self.ch);
            }
            ')' => {
                new_token = Token::RPAREN(self.ch)
            }
            ',' => {
                new_token = Token::COMMA(self.ch)
            }
            '+' => {
                new_token = Token::PLUS(self.ch)
            }
            '{' => {
                new_token = Token::LBRACE(self.ch)
            }
            '}' => {
                new_token = Token::RBRACE(self.ch)
            }
            _ => {
                if is_letter(self.ch) {
                    //return (self, Token::new(TokenType::ILLEGAL, self.read_identifier()))
                    return Token::ILLEGAL;
                } else {
                    //new_token = Token::new(TokenType::ILLEGAL, String::from(self.ch))
                    return Token::ILLEGAL;
                }
            }
        }

        self.read_char();
        return new_token;
    }
}


fn is_letter(ch: char) -> bool {
    return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_';
}

#[test]
fn test_next_token() {
    let input = String::from("=+(){},;");

    let expected_tokens = [
        Token::EQUAL('='),
        Token::PLUS('+'),
        Token::LPAREN('('),
        Token::RPAREN(')'),
        Token::LBRACE('{'),
        Token::RBRACE('}'),
        Token::COMMA(','),
        Token::SEMICOLON(';'),
    ];

    println!("========== TEST 1 =========");
    let mut l = Lexer::new(input.chars().collect());
    l.read_char();

    let mut actual: Token;
    for expected_token in expected_tokens.into_iter() {
        actual = l.next_token();

        assert_eq!(expected_token, actual);
    }
}

#[test]
fn test_next_token_extended() {
    let input = String::from("let five = 5;
                             let ten = 10;
                             
                             let add = fn(x, y) {
                               x + y;
                             };

                             let result = add(five, ten);
                             ");

    let expected_tokens = [
        Token::LET("let".chars().collect()),
        Token::IDENT("five".chars().collect()),
        Token::EQUAL('='),
        Token::INT("5".chars().collect()), 
        Token::SEMICOLON(';'),
        Token::LET("let".chars().collect()),
        Token::IDENT("ten".chars().collect()),
        Token::EQUAL('='),
        Token::INT("10".chars().collect()),
        Token::SEMICOLON(';'),
        Token::LET("let".chars().collect()),
        Token::IDENT("add".chars().collect()),
        Token::EQUAL('='),
        Token::FUNCTION("fn".chars().collect()),
        Token::LPAREN('('),
        Token::IDENT("x".chars().collect()),
        Token::COMMA(','),
        Token::IDENT("y".chars().collect()),
        Token::RPAREN(')'),
        Token::LBRACE('{'),
        Token::IDENT("x".chars().collect()),
        Token::PLUS('+'),
        Token::IDENT("x".chars().collect()),
        Token::SEMICOLON(';'),
        Token::RBRACE('}'),
        Token::SEMICOLON(';'),
        Token::LET("let".chars().collect()),
        Token::IDENT("result".chars().collect()),
        Token::EQUAL('='),
        Token::IDENT("add".chars().collect()),
        Token::LPAREN('('),
        Token::IDENT("five".chars().collect()),
        Token::COMMA(','),
        Token::IDENT("ten".chars().collect()),
        Token::RPAREN(')'),
        Token::SEMICOLON(';'),
        Token::EOF,
    ];
    println!("========== TEST 2 =========");
    let mut l = Lexer::new(input.chars().collect());
    l.read_char();

    let mut actual: Token;
    for expected_token in expected_tokens.into_iter() {
        actual = l.next_token();

        assert_eq!(expected_token, actual);
    }
}

