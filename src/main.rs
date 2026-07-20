use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Token {
    Str(String),
    Num(i32),
    LBrace,
    RBrace,
    DDot,
}

//Custom tokens for the lexer
impl From<Token> for String {
    fn from(token: Token) -> Self {
        match token {
            Token::LBrace => "{".to_string(),
            Token::RBrace => "}".to_string(),
            Token::Str(s) => s,
            Token::DDot => ':'.to_string(),
            _ => panic!("not a token type"),
        }
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        Token::Str(value)
    }
}

impl From<i32> for Token {
    fn from(value: i32) -> Self {
        Token::Num(value)
    }
}

impl PartialEq<String> for Token {
    fn eq(&self, other: &String) -> bool {
        match self {
            Token::LBrace => other == "{",
            Token::RBrace => other == "}",
            Token::DDot => other == ":",
            _ => false,
        }
    }
}

// Custom errors for easier type matching
#[derive(Debug)]
enum JsonError {
    IOError,
    UTF8Error,
}

impl From<std::io::Error> for JsonError {
    fn from(_error: std::io::Error) -> Self {
        JsonError::IOError
    }
}

impl From<std::str::Utf8Error> for JsonError {
    fn from(_error: std::str::Utf8Error) -> Self {
        JsonError::UTF8Error
    }
}

fn lexer(file: &mut BufReader<File>) -> Result<Vec<Token>, JsonError> {
    let mut token_vec = vec![];
    loop {
        let mut buf = vec![0u8; 1];
        if file.read(&mut buf)? == 0 {
            break;
        }
        let temp = str::from_utf8(&buf)?;

        if temp == "{" {
            token_vec.push(Token::LBrace);
        } else if temp == "}" {
            token_vec.push(Token::RBrace);
        } else if temp == "\"" {
            token_vec.push(str_lexer(file)?);
        } else if temp == ":" {
            token_vec.push(Token::DDot);
        } else if temp.as_bytes()[0].is_ascii_digit() {
            token_vec.push(num_lexer(file, temp.parse().unwrap())?);
        }
    }
    Ok(token_vec)
}

fn str_lexer(file: &mut BufReader<File>) -> Result<Token, JsonError> {
    let mut str: String = "".to_string();
    loop {
        let mut buf = [0u8];
        if file.read(&mut buf)? == 0 {
            break;
        }
        let temp = str::from_utf8(&buf)?;
        if temp == "\"" {
            break;
        }
        str += temp;
    }
    Ok(Token::Str(str))
}

fn num_lexer(file: &mut BufReader<File>, firstnum: i32) -> Result<Token, JsonError> {
    let mut str = firstnum.to_string();
    loop {
        let mut buf = [0u8];
        if file.read(&mut buf)? == 0 {
            break;
        }
        let temp = str::from_utf8(&buf)?;
        if !temp.as_bytes()[0].is_ascii_digit() {
            break;
        }
        str += temp;
    }
    Ok(Token::Num(str.parse().unwrap()))
}

fn parser(
    tvec: &Vec<Token>,
    mut index: i8,
    myobj: &mut HashMap<String, Token>,
) -> Result<(), JsonError> {
    loop {
        index += 1;
        if tvec[index as usize] == "{".to_string() {
            parser(tvec, index, myobj)?;
        } else if tvec[index as usize] == "}".to_string() {
            break;
        } else if tvec[index as usize] == Token::DDot {
            myobj.insert(
                String::from(tvec[(index - 1) as usize].clone()),
                tvec[(index + 1) as usize].clone(),
            );
        }
    }
    Ok(())
}

fn main() -> Result<(), JsonError> {
    let file = File::open("./src/test.json")?;
    let mut buf = BufReader::new(file);
    let token_vec = lexer(&mut buf)?;
    let mut mymap = HashMap::new();
    parser(&token_vec, 0, &mut mymap)?;
    println!("{:?}", mymap);
    Ok(())
}