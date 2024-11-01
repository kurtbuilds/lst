use std::{
    io::{self, Write},
    mem::take,
};

fn main() {
    println!("Parsing repl. Enter a Json string or fragment. Two empty lines to parse.");

    let mut empty = 0;
    let mut buf = String::new();

    loop {
        // Print prompt
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        buf.push_str(&line);
        if line.trim_end().is_empty() {
            empty += 1;
        } else {
            empty = 1;
        }
        if empty >= 2 {
            let result = lst::parse(take(&mut buf));
            dbg!(result);
        }
    }
}
