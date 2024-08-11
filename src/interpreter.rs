use std::{fs, str::Chars};

use crate::cli::RunArgs;

const VALID_CHARS: [char; 8] = ['+', '-', '>', '<', '.', ',', '[', ']'];

#[derive(Debug)]
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
    if chars.len() <= 0 {
        return Err("No valid symbols in given file");
    }
    let ops = match get_ops(chars) {
        Ok(ops) => ops,
        Err(e) => return Err(e),
    };

    if debug {
        for op in ops {
            println!("Operation: {:?} - Value: {:?}", op.kind, op.value);
        }
    }

    return Ok(());
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

    let mut seen_open_bracket = false;

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
                seen_open_bracket = true;
                ops.push(Op {
                    kind: OpKind::OpStart,
                    value: 0,
                })
            }
            ']' => {
                if !seen_open_bracket {
                    return Err("Closing bracket found before opening bracket");
                }
                ops.push(Op {
                    kind: OpKind::OpEnd,
                    value: 0,
                });
                seen_open_bracket = true;
            }
            _ => (),
        }
        if i < chars.len() - 1 {
            i += 1;
        } else {
            break;
        };
        c = chars[i];
    }
    Ok(ops)
}
