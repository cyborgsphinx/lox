use std::env;

mod scanner;

fn run(args: Vec<String>) -> Result<(), i32> {
    match args.len() {
        1 => run_repl(),
        2 => interpret_file(&args[1]),
        _ => {eprintln!("Usage: rlox [file]"); Err(64)},
    }
}

fn run_repl() -> Result<(), i32> {
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
