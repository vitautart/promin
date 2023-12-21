use std::fs;

#[derive(Debug)]
enum Token
{
    Semicolon,
    Comma,
    Colon,

    Minus,
    Plus,
    Divide,
    Star,
    Amp,
    Asign,

    Arrow,

    LRoundBracket,
    RRoundBracket,
    LSquareBracket,
    RSquareBracket,
    LCurlyBracket,
    RCurlyBracket,
    LAngleBracket,
    RAngleBracket,

    Func,
    Let,
    Struct,
    Return,

    Identifier(String),
    NumberGeneral(String),
    //NumberF64(f64),
    //NumberU64(u64),
    //NumberF64(f64),
    String(String),
    Char(String),

    Comment,
}

fn lex_keyword(name: &str) -> Token
{
    match name
    {
        "let" => Token::Let,
        "fn" => Token::Func,
        "return" => Token::Return,
        "struct" => Token::Struct,
        _ => Token::Identifier(String::from(name)),
    }
}

fn is_identifier_char(c: char) -> bool
{
    c.is_alphanumeric() || c == '_' 
}

fn is_start_identifier_char(c: char) -> bool
{
    c.is_alphabetic() || c == '_' 
}

fn lex(filedata: &String) -> Vec<Token>
{
    let mut tokens = Vec::<Token>::new();
    let mut state: Option<Token> = None;
    for (_n, c) in filedata.char_indices() 
    {
        if let Some(Token::Comment) = state
        { 
            if c == '\n' 
            { 
                state = None; 
            }
            continue;
        }
        else if let Some(Token::Divide) = state
        {
            if c == '/' 
            {
                tokens.pop(); 
                state = Some(Token::Comment); 
                continue;
            }
        }
        else if let Some(Token::Minus) = state 
        {
            if c == '>' 
            {
                tokens.pop(); 
                tokens.push(Token::Arrow); 
                state = None; 
                continue;
            }
        }
        else if let Some(Token::Identifier(ref mut name)) = state 
        {
            if is_identifier_char(c) 
            {
                name.push(c);
                continue;
            }
            else 
            { 
                tokens.push(lex_keyword(name.as_str())); 
            }
        }
        else if let Some(Token::NumberGeneral(ref mut name)) = state 
        {
            if is_identifier_char(c)
            {
                name.push(c);
                continue;
            }
            else
            { 
                tokens.push(Token::NumberGeneral(name.clone())); 
            }
        }
        // TODO: it is more like raw string that actual string with /n and so on
        else if let Some(Token::String(ref mut literal)) = state
        {
            if c != '"'
            { 
                literal.push(c); 
            }
            else 
            { 
                tokens.push(Token::String(literal.clone())); 
                state = None; 
            }
            continue;
        }
        else if let Some(Token::Char(ref mut literal)) = state
        {
            if c != '\''
            { 
                literal.push(c); 
            }
            else 
            { 
                tokens.push(Token::Char(literal.clone())); 
                state = None; 
            }
            continue;
        }

        // Reset state
        state = None;

        if c.is_whitespace() { }
        else if is_start_identifier_char(c) { state = Some(Token::Identifier(String::from(c))); }
        else if c.is_ascii_digit() { state = Some(Token::NumberGeneral(String::from(c))); }

        else if c == ';' { tokens.push(Token::Semicolon); }
        else if c == ',' { tokens.push(Token::Comma); }
        else if c == '(' { tokens.push(Token::LRoundBracket); }
        else if c == ')' { tokens.push(Token::RRoundBracket); }
        else if c == '[' { tokens.push(Token::LSquareBracket); }
        else if c == ']' { tokens.push(Token::RSquareBracket); }
        else if c == '{' { tokens.push(Token::LCurlyBracket); }
        else if c == '}' { tokens.push(Token::RCurlyBracket); }

        else if c == '=' { tokens.push(Token::Asign); }
        else if c == ':' { tokens.push(Token::Colon); }
        else if c == '<' { tokens.push(Token::LAngleBracket); }
        else if c == '>' { tokens.push(Token::RAngleBracket); }
        else if c == '+' { tokens.push(Token::Plus); }
        else if c == '*' { tokens.push(Token::Star); }
        else if c == '&' { tokens.push(Token::Amp); }

        else if c == '-' { tokens.push(Token::Minus); state = Some(Token::Minus); }
        else if c == '/' { tokens.push(Token::Divide); state = Some(Token::Divide); }
        else if c == '"' { state = Some(Token::String(String::new())); }
        else if c == '\'' { state = Some(Token::Char(String::new())); }
    }

    tokens
}

//fn parse(tokens: &Vec<Token>) -> () {}

fn main() -> () 
{
    println!("[INFO] Start compiler");
    let filepath = "test.min";
    let filedata = fs::read_to_string(filepath);
    let filedata = match filedata 
    {
        Ok(f) => f,
        Err(err) => { eprintln!("[ERROR] {err}"); String::new() },
    };
    println!("{filedata}");

    let tokens = lex(&filedata);
    println!("{:?}", tokens);
}
