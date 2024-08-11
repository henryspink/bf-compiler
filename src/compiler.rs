use crate::cli::CompileArgs;
use std::fs;

pub fn compile(args: CompileArgs) -> Result<(), &'static str> {
    let path = args.path;
    let emit = args.emit;
    let optimize = !args.no_optimize;

    // if true {
    //     return Err("Not implemented");
    // }

    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(_) => return Err("Failed to read file"),
    };

    match emit.to_ascii_lowercase().as_str() {
        "asm" => compile_to_asm(source, optimize),
        "binary" => compile_to_binary(source, optimize),
        _ => Err("Invalid emit type"),
    }
}

fn compile_to_asm(source: String, optimize: bool) -> Result<(), &'static str> {
    if optimize {
        for c in source.chars() {
            // match c {
            //     '+' => println!("inc rax"),
            //     '-' => println!("dec rax"),
            //     '>' => println!("inc rdi"),
            //     '<' => println!("dec rdi"),
            //     '.' => println!("call putchar"),
            //     ',' => println!("call getchar"),
            //     '[' => println!("loop_start:"),
            //     ']' => println!("loop_end:"),
            //     _ => (),
            // }
            println!("{}", c);
        }
        return Err("Optimized version is not implemented");
    } else {
        return Err("Non-optimized version is not implemented");
    }
}

fn compile_to_binary(source: String, optimize: bool) -> Result<(), &'static str> {
    if optimize {
        match compile_to_asm(source, optimize) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        // do stuff
        return Err("Optimized version is not implemented");
    } else {
        return Err("Non-optimized version is not implemented");
    }
}
