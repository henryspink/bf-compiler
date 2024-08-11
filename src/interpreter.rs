use std::{fs, str::Chars};

use crate::cli::RunArgs;

const VALID_CHARS: [char; 8] = ['+', '-', '>', '<', '.', ',', '[', ']'];

#[derive(Debug, PartialEq)]
#[repr(u8)]
enum OpKind {
    OpInc = b'+',
    OpDec = b'-',
    OpRight = b'>',
    OpLeft = b'<',
    OpOut = b'.',
    OpIn = b',',
    OpStart = b'[',
    OpEnd = b']',
}

impl OpKind {
    fn from_char(c: char) -> Result<OpKind, ()> {
        match c {
            '+' => Ok(OpKind::OpInc),
            '-' => Ok(OpKind::OpDec),
            '>' => Ok(OpKind::OpRight),
            '<' => Ok(OpKind::OpLeft),
            '.' => Ok(OpKind::OpOut),
            ',' => Ok(OpKind::OpIn),
            '[' => Ok(OpKind::OpStart),
            ']' => Ok(OpKind::OpEnd),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Op {
    kind: OpKind,
    value: isize,
}

pub fn interpret(args: RunArgs) -> Result<(), &'static str> {
    let path = args.path;
    let debug = args.debug;

    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(_) => return Err("Failed to read file"),
    };

    let chars = lexed(source.chars());
    if debug {
        let mut count = 0;
        for c in &chars {
            println!("{}: {}", count, c);
            count += 1;
        }
        println!();
    }

    if chars.len() <= 0 {
        return Err("No valid symbols in given file");
    }

    let ops = match get_ops(chars) {
        Ok(ops) => ops,
        Err(e) => return Err(e),
    };

    if debug {
        for op in &ops {
            println!("Operation: {:?} - Value: {:?}", op.kind, op.value);
        }
    }

    return run(ops);
    // return Ok(());
}

fn lexed(chars: Chars) -> Vec<char> {
    let mut lexed = Vec::new();

    for char in chars {
        if VALID_CHARS.contains(&char) {
            lexed.push(char);
        }
    }

    lexed
}

fn get_ops(chars: Vec<char>) -> Result<Vec<Op>, &'static str> {
    let mut ops: Vec<Op> = Vec::new();

    let mut open_brackets = 0;
    let mut close_brackets = 0;

    let mut i = 0;
    let mut c = chars[i];

    while i < chars.len() {
        match c {
            '+' | '-' | '>' | '<' | '.' | ',' => {
                let mut count = 0;
                let mut nextc = chars[i];
                while i < chars.len() {
                    if nextc == c {
                        count += 1;
                    } else {
                        i -= 1;
                        break;
                    }

                    if i < chars.len() - 1 {
                        i += 1;
                    } else {
                        break;
                    };

                    nextc = chars[i]
                }
                ops.push(Op {
                    kind: OpKind::from_char(c).unwrap(),
                    value: count,
                });
            }
            '[' => {
                open_brackets += 1;
                let mut j = i;
                let mut skip_times = 0;
                loop {
                    j += 1;
                    if j >= chars.len() {
                        return Err("No closing bracket found");
                    }
                    if chars[j] == '[' {
                        skip_times += 1;
                    } else if chars[j] == ']' {
                        if skip_times == 0 {
                            break;
                        } else {
                            skip_times -= 1;
                        }
                    }
                }
                ops.push(Op {
                    kind: OpKind::OpStart,
                    value: j as isize,
                })
            }
            ']' => {
                close_brackets += 1;
                if open_brackets < close_brackets {
                    return Err("Closing bracket found before opening bracket");
                }
                let mut j = i;
                let mut skip_times = 0;
                loop {
                    // println!("{} j", j);
                    if j <= 0 {
                        // println!("{} jy {}", j, chars[j]);
                        return Err("No opening bracket found");
                    }
                    j -= 1;
                    if chars[j] == ']' {
                        skip_times += 1;
                    } else if chars[j] == '[' {
                        if skip_times == 0 {
                            break;
                        } else {
                            skip_times -= 1;
                        }
                    }
                    // println!("{} skip", skip_times);
                }
                ops.push(Op {
                    kind: OpKind::OpEnd,
                    value: j as isize,
                });
            }
            _ => (),
        }
        if i < chars.len() - 1 {
            i += 1;
        } else {
            break;
        };
        c = chars[i];
        // println!("{}, {}", open_brackets, close_brackets);
    }
    Ok(ops)
}

// fn run(ops: Vec<Op>) -> Result<(), &'static str> {
//     let mut mem: Vec<u8> = vec![0; 36];
//     let mut pointer = 16;
//     let mut i = 0;

//     while i < ops.len() {
//         let op = &ops[i];
//         println!("{}: {:?} - {}", i, op.kind, op.value);
//         // println!("{:?}", mem);
//         match op.kind {
//             OpKind::OpInc => mem[pointer] += op.value as u8,
//             OpKind::OpDec => mem[pointer] -= op.value as u8,
//             OpKind::OpRight => pointer += op.value as usize,
//             OpKind::OpLeft => pointer -= op.value as usize,
//             OpKind::OpOut => for _ in 0..op.value {print!("{}", mem[pointer] as char)},
//             OpKind::OpIn => {
//                 let mut input = String::new();
//                 std::io::stdin().read_line(&mut input).unwrap();
//                 mem[pointer] = input.chars().next().unwrap() as u8;
//             },
//             OpKind::OpStart => {
//                 if mem[pointer] == 0 {
//                     i = op.value as usize;
//                     // i += 1;
//                 }
//             },
//             OpKind::OpEnd => {
//                 if mem[pointer] != 0 {
//                     i = op.value as usize;
//                     // i += 1;
//                 }
//             },
//         }
//         println!("{}, {}", i, pointer);
//         if op.kind != OpKind::OpEnd || mem[pointer] == 0 {
//             i += 1;
//         }
//     }
//     Ok(())
// }

fn run(ops: Vec<Op>) -> Result<(), &'static str> {
    let mut mem: Vec<u8> = vec![0; 30000];
    let mut pointer = 0;
    let mut i = 0;

    while i < ops.len() {
        let op = &ops[i];

        println!("Executing {:?} at index {}. Pointer: {}. Mem[pointer]: {}", op.kind, i, pointer, mem[pointer]);

        match op.kind {
            OpKind::OpInc => mem[pointer] = mem[pointer].wrapping_add(op.value as u8),
            OpKind::OpDec => mem[pointer] = mem[pointer].wrapping_sub(op.value as u8),
            OpKind::OpRight => {
                pointer = pointer.wrapping_add(op.value as usize);
                if pointer >= mem.len() {
                    return Err("Memory pointer overflow");
                }
            },
            OpKind::OpLeft => {
                if pointer < op.value as usize {
                    println!("{}", op.value);
                    return Err("Memory pointer underflow");
                }
                pointer = pointer.wrapping_sub(op.value as usize);
            },
            OpKind::OpOut => for _ in 0..op.value { print!("{}", mem[pointer] as char); },
            OpKind::OpIn => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                mem[pointer] = input.chars().next().unwrap() as u8;
            },
            OpKind::OpStart => {
                if mem[pointer] == 0 {
                    i = op.value as usize;
                }
            },
            OpKind::OpEnd => {
                if mem[pointer] != 0 {
                    i = op.value as usize;
                }
            },
        }

        // if op.kind != OpKind::OpEnd || mem[pointer] == 0 {
        //     i += 1;
        // }
        i += 1;
    }

    Ok(())
}
