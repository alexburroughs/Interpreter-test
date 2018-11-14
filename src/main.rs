use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    // Get arguments (source code path)
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Open file
    let mut f = File::open(filename).expect("file not found");

    // Read all source code into a string
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("error reading file");

    // Hack to make parsing easier
    contents.push(' ');

    let tokens = parse_file(contents);

    run(tokens);
}

fn parse_file(code : String) -> Vec<Token> {

    use State::*;

    // Get source code string as chars
    let chars : Vec<char> = code.chars().collect();

    // Empty list of tokens to be populated and returned
    let mut tokens : Vec<Token> = Vec::new();

    // Counter for loop, line counter, id and current state
    let mut counter = 0;
    let mut line = 1;
    let mut id = 0;
    let mut state = State::START;

    // Current text being processed
    let mut current = String::from("");

    // loop through all chars in the source code
    while counter < chars.len() {

        if let Some(ref tmp) = chars.get(counter) {

            match state {
                START => {
                    if tmp.is_digit(10) {

                        // temporary clone of tmp so it can be used TODO: maybe delete?
                        let curr = tmp.clone();
                        current.push(*curr);

                        // set the state
                        state = NUMBER;
                    }
                    else if tmp.is_whitespace() {

                        // TODO delete and see what happens
                    }
                    else if tmp.is_ascii_alphabetic() {
                        
                        // push the current char to the current token
                        let curr = tmp.clone();
                        current.push(*curr);

                        // set the state
                        state = KEY;
                    }

                    else {
                        panic!("Error in parsing");
                    }
                },
                
                NUMBER => {
                    if tmp.is_digit(10) {

                        // Continue pushing chars onto the current token
                        let curr = tmp.clone();
                        current.push(*curr);
                    }

                    // If the current token is done
                    else if tmp.is_whitespace() {

                        // Make the token and push onto tokens list
                        let curr_token = Token {
                            id : id,
                            key : Keyword::NUMBER,
                            text : current.clone(),
                            line : line
                        };

                        //println!("line: {} text: {} id: {}\n", &line, &curr_token.text, &id);
                        tokens.push(curr_token);

                        // Reset the current token and state
                        current = String::new();
                        state = START;

                        line += 1;
                        id += 1;
                    }

                    // Error in source code (garbage programmer)
                    else {
                        panic!("Error in parsing");
                    }
                },
                KEY => {
                    if tmp.is_ascii_alphabetic() {

                        // Continue pushing chars onto the current token
                        let curr = tmp.clone();
                        current.push(*curr);
                    }

                    // If the current token is done
                    else if tmp.is_whitespace() {

                        let token_type : Keyword;

                        let mut line_add = 0;

                        // Make the token and push onto tokens list
                        // set if the line will be incremented based on the 
                        match current.as_ref() {
                            "push" => {token_type = Keyword::PUSH},
                            "pop" => {token_type = Keyword::POP; line_add += 1},
                            "add" => {token_type = Keyword::ADD; line_add += 1},
                            "ifeq" => {token_type = Keyword::IFEQ; line_add += 1},
                            "jump" => {token_type = Keyword::JUMP},
                            "print" => {token_type = Keyword::PRINT; line_add += 1},
                            "dup" => {token_type = Keyword::DUP; line_add += 1},
                            _ => panic!("Error")
                        }

                        // Make the token and push onto tokens list
                        let curr_token = Token {
                            id : id,
                            key : token_type,
                            text : current.clone(),
                            line : line
                        };

                        //println!("line: {} text: {} id: {}\n", &line, &curr_token.text, &id);
                        tokens.push(curr_token);

                        // Reset the current token and state
                        current = String::new();
                        state = START;

                        line += line_add;
                        id += 1;
                    }

                    // Error in source code (garbage programmer)
                    else {
                        panic!("Error in parsing");
                    }
                }
            }
        }

        counter += 1;
    }
    return tokens;
}

fn run (tokens : Vec<Token>) {

    use Keyword::*;

    let mut num_stack : Vec<i32> = Vec::new();

    let mut current_id = 0;

    while current_id < tokens.len() {

        // Stuff I have to do but dont like
        if let Some(ref token) = tokens.get(current_id) {

            match &token.key {
                PUSH => {

                    current_id += 1;

                    if let Some(ref num) = tokens.get(current_id) {
                        num_stack.push(num.text.parse::<i32>().unwrap());
                    }
                    current_id += 1;
                },
                POP => {
                    if num_stack.len() < 1{
                        panic!("Runtime Error: too much poping");
                    }
                    
                    num_stack.pop();

                    current_id += 1;
                },
                ADD => {
                    //println!("len: {}", num_stack.len());
                    if num_stack.len() < 2 {
                        panic!("Runtime Error: too much poping");
                    }

                    let x : i32;
                    let y : i32;

                    match num_stack.pop() {
                        Some(ref a) => {x = *a},
                        None => panic!()
                    }

                    match num_stack.pop() {
                        Some(ref a) => {y = *a},
                        None => panic!()
                    }

                    num_stack.push(x + y);
                    
                    current_id += 1;
                },
                IFEQ => {

                    if num_stack.len() < 1{
                        panic!("Runtime Error: too much poping");
                    }

                    let x : i32;

                    match num_stack.pop() {
                        Some(ref a) => {x = *a},
                        None => panic!()
                    }

                    if x == 0 {
                        current_id += 1;
                    }
                    else {
                        current_id += 1;
                        // Get the address token
                        if let Some(ref num) = tokens.get(current_id) {

                            current_id = get_address(num.text.parse::<i32>().unwrap(), &tokens);
                        }
                    }
                    continue;
                },
                JUMP => {
                    current_id += 1;
                    // Get the address token
                    if let Some(ref num) = tokens.get(current_id) {

                        current_id = get_address(num.text.parse::<i32>().unwrap(), &tokens);
                    }

                    continue;
                },
                PRINT => {
                    if num_stack.len() < 1{
                        panic!("Runtime Error: not enough pushing");
                    }
                    
                    // TODO replace
                    // LAZY CODE ----
                    //              V

                    let x : i32;

                    match num_stack.pop() {
                        Some(ref a) => {x = *a},
                        None => panic!()
                    }

                    println!("{}", x);

                    num_stack.push(x);

                    // END OF LAZY CODE
                    current_id += 1;
                },
                DUP => {
                    if num_stack.len() < 1{
                        panic!("Runtime Error: not enough pushing");
                    }
                    
                    // TODO replace
                    // MORE LAZY CODE ----
                    //                   V

                    let x : i32;

                    match num_stack.pop() {
                        Some(ref a) => {x = *a},
                        None => panic!()
                    }

                    num_stack.push(x);
                    num_stack.push(x);

                    // END OF LAZY CODE
                    current_id += 1;
                },
                _ => panic!("Runtime Error")
            }
        }
    } 
}

fn get_address(address: i32, tokens : &Vec<Token>) -> usize {

    for x in tokens {
        match x.key {
            Keyword::NUMBER => continue,
            _ => {
                if x.line == address {
                    return x.id;
                }
            }
        }
    }

    panic!("Runtime Error: invalid line number (lines start at 1)");
}

struct Token {
    id : usize,
    key : Keyword,
    text : String,
    line : i32
}

enum Keyword {
    PUSH,
    POP,
    ADD,
    IFEQ,
    JUMP,
    PRINT,
    DUP,
    NUMBER
}

enum State {
    START,
    NUMBER,
    KEY
}