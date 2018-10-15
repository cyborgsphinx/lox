extern crate rlox;
use rlox::scanner;

use std::{io, env};
use std::io::{Write, BufRead};

fn run(args: Vec<String>) -> Result<(), i32> {
    match args.len() {
        1 => run_repl(),
        2 => interpret_file(&args[1]),
        _ => {
            eprintln!("Usage: rlox [file]");
            Err(64)
        }
    }
}

fn run_repl() -> Result<(), i32> {
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        let mut buf = String::new();
        let stdin = io::stdin();
        let val = stdin.lock().read_line(&mut buf);
        match val {
            Err(e) => return Err(e.raw_os_error().unwrap_or(-1)),
            Ok(0) => break,
            _ => {}
        }
        for token in scanner::Scanner::new(&buf) {
            println!("{:?}", token);
        }
    }
    Ok(())
}

fn interpret_file(file_name: &str) -> Result<(), i32> {
    let _ = file_name;
    Ok(())
}

fn main() {
    ::std::process::exit(match run(env::args().collect()) {
        Ok(..) => 0,
        Err(e) => e,
    });
}
