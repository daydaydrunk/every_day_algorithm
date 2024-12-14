use std::iter::Peekable;
use std::str::Chars;

// Token definitions
#[derive(Debug, PartialEq, Clone)]
enum Token {
    Select,
    From,
    Where,
    Identifier(String),
    Number(f64),
    String(String),
    Comma,
    Operator(String),
    EOF,
}

// AST Node definitions
#[derive(Debug)]
struct SelectStatement {
    columns: Vec<String>,
    table: String,
    condition: Option<Condition>,
}

#[derive(Debug)]
struct Condition {
    left: String,
    operator: String,
    right: String,
}

struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.input.peek() {
            None => Token::EOF,
            Some(&c) => match c {
                ',' => {
                    self.input.next();
                    Token::Comma
                }
                'a'..='z' | 'A'..='Z' => {
                    let ident = self.read_identifier();
                    match ident.to_uppercase().as_str() {
                        "SELECT" => Token::Select,
                        "FROM" => Token::From,
                        "WHERE" => Token::Where,
                        _ => Token::Identifier(ident),
                    }
                }
                '0'..='9' => {
                    let num = self.read_number();
                    Token::Number(num)
                }
                _ => {
                    self.input.next();
                    Token::Operator(c.to_string())
                }
            },
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.input.next();
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(&c) = self.input.peek() {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            ident.push(c);
            self.input.next();
        }
        ident
    }

    fn read_number(&mut self) -> f64 {
        let mut num_str = String::new();
        while let Some(&c) = self.input.peek() {
            if !c.is_numeric() && c != '.' {
                break;
            }
            num_str.push(c);
            self.input.next();
        }
        num_str.parse().unwrap_or(0.0)
    }
}

struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
        }
    }

    fn parse_select(&mut self) -> Option<SelectStatement> {
        // Expect SELECT
        if self.current_token != Token::Select {
            return None;
        }
        self.next_token();

        // Parse columns
        let mut columns = Vec::new();
        loop {
            if let Token::Identifier(col) = self.current_token.clone() {
                columns.push(col);
                self.next_token();

                if self.current_token != Token::Comma {
                    break;
                }
                self.next_token();
            } else {
                break;
            }
        }

        // Expect FROM
        if self.current_token != Token::From {
            return None;
        }
        self.next_token();

        // Parse table name
        let table = if let Token::Identifier(table) = self.current_token.clone() {
            table
        } else {
            return None;
        };
        self.next_token();

        // Parse WHERE clause if exists
        let condition = if self.current_token == Token::Where {
            self.next_token();
            Some(self.parse_condition())
        } else {
            None
        };

        Some(SelectStatement {
            columns,
            table,
            condition,
        })
    }

    fn parse_condition(&mut self) -> Condition {
        let left = if let Token::Identifier(left) = self.current_token.clone() {
            left
        } else {
            String::new()
        };
        self.next_token();

        let operator = if let Token::Operator(op) = self.current_token.clone() {
            op
        } else {
            String::new()
        };
        self.next_token();

        let right = if let Token::Identifier(right) = self.current_token.clone() {
            right
        } else {
            String::new()
        };
        self.next_token();

        Condition {
            left,
            operator,
            right,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }
}

// Public interface
pub fn parse_sql(sql: &str) -> Option<SelectStatement> {
    let mut parser = Parser::new(sql);
    parser.parse_select()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_select() {
        let sql = "SELECT name, age FROM users";
        let ast = parse_sql(sql);
        assert!(ast.is_some());
        let stmt = ast.unwrap();
        assert_eq!(stmt.columns, vec!["name", "age"]);
        assert_eq!(stmt.table, "users");
        assert!(stmt.condition.is_none());
    }

    #[test]
    fn test_select_with_where() {
        let sql = "SELECT name FROM users WHERE age=20";
        let ast = parse_sql(sql);
        assert!(ast.is_some());
        let stmt = ast.unwrap();
        assert_eq!(stmt.columns, vec!["name"]);
        assert_eq!(stmt.table, "users");
        assert!(stmt.condition.is_some());
    }
}
