use std::fs::File;
use std::io::Read;
use regex::Regex;

fn lexer(content: String) {
    let lines = content.split('\n');

    for line in lines {
        let chars = line.chars();

        let mut tokens: Vec<String> = Vec::new();
        let mut temp_s: String      = String::new();
        let mut quotes: i8          = 0;
        let mut is_str: bool;

        for character in chars {
            if character == '"' {
                quotes += 1;
            }

            match quotes % 2 {
                0 => is_str = false,
                _ => is_str = true
            }

            if character == ' ' && is_str == false {
                tokens.push(temp_s);
                temp_s = String::new();
            }
            else {
                temp_s += &String::from(character);
            }
        }
        tokens.push(temp_s);

        let mut items = Vec::new();

        for token in &tokens {
            if &token[0..1] == "\"" {
                match &token[token.len() - 1..] {
                    "\"" => items.push(("string", token)),
                    _    => {
                        println!("An error occured: no end of string"); 
                        break;
                    }
                }
            }
            else if Regex::new(r"[.a-zA-Z]+").unwrap().is_match(&token) {
                items.push(("symbol", token));
            }
            else if String::from("+-*/").contains(&*token) {
                items.push(("expression", token));
            }
            else if Regex::new(r"[.0-9]+").unwrap().is_match(&token) {
                items.push(("number", token));
            }
            else if token == "->" {
                items.push(("assign", token));
            }
        }

        //println!("{:?}", tokens);
        println!("{:#?}", items);
    }
}

pub fn parse(path: String) {
    let mut file: File = File::open(path).expect("An error occured while opening file");
    let mut content: String = String::new();
    
    file.read_to_string(&mut content).expect("An error occured while reading file");

    lexer(content);

    //let tokens = lexer(content);

    //println!("{}", tokens);
}

/*
Token Types

string -> "Hello, world"
symbol -> func, var ...
expres -> + - * / ** == != > < >= <= -> <- %
number -> 10, 2.5, 0x00, 010101
*/