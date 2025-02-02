use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::str::FromStr;

use simplevm::{Instruction, InstructionParseError};

fn main() -> Result<(), String> {
    // ./asm file.asm

    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: {} <input>", args[0]);
    }

    let file = File::open(Path::new(&args[1])).map_err(|x| format!("failed to open: {}", x))?;
    let mut output: Vec<u8> = Vec::new();
    /*
     * Push 10
     * Push 240
     * AddStack
     * PopRegister A
     * Signal $f0
     *
     */
    for line in BufReader::new(file).lines() {
        let line_inner = line.map_err(|_x| "foo")?;
        if line_inner.is_empty() {
            continue;
        }
        if line_inner.chars().next().unwrap() == ';' {
            continue;
        }
        match Instruction::from_str(&line_inner) {
            Ok(instruction) => {
                let raw_instruction: u16 = instruction.encode_u16();
                // assumption: >>8 needs to mask for u16
                output.push((raw_instruction & 0xff) as u8);
                output.push((raw_instruction >> 8) as u8);
            }
            Err(InstructionParseError::Fail(s)) => {
                panic!("{}", s);
            }
            _ => continue,
        }
    }
    let mut stdout = io::stdout().lock();
    stdout.write_all(&output).map_err(|x| format!("{}", x))?;
    Ok(())
}
