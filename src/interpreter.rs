use std::fs::File;
use std::io::Read;

fn lexer(content: String) {
    let lines = content.split('\n');

    for line in lines {
        let chars = line.chars();

        let mut tokens: Vec<String> = Vec::new();
        let mut temp_s: String      = String::new();
        let mut quotes: i8          = 0;
        let mut is_str: bool        = false;

        for character in chars {
            if character == '"' {
                quotes += 1;
            }

            if quotes % 2 == 0 {
                is_str = false;
            }
            else {
                is_str = true;
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
        println!("{:?}", tokens);
    }
}

pub fn parse(path: String) {
    let mut file: File = File::open(path).expect("An error occured while opening file");
    let mut content: String = String::new();
    
    file.read_to_string(&mut content).expect("An error occured while reading file");

    let tokens = lexer(content);

    //println!("{}", tokens);
} 