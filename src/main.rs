use std::fs;

const ZERO: char = ' ';
const ONE: char = '\t';
const IGNORE_NEWLINES: bool = false;


#[derive(Debug)]
enum Instruction {
    PointerRight,
    PointerLeft,
    Increase,
    Decrease,
    Print,
    Input,
    LoopBegin,
    LoopEnd,
}

fn file_to_instruction(f: String) -> Vec<Instruction> {
    let mut counter: u8 = 0;
    let mut buffer: String = String::new();
    let mut result: Vec<Instruction> = vec![];
    let mut file_content = f;
    if IGNORE_NEWLINES {
        file_content = file_content.replace("\n", "");
    }

    for c in file_content.chars() {
        if c == ZERO || c == ONE {
            counter += 1;

            match counter {
                1 | 2 => {
                    buffer.push(c);
                },
                3 => {
                    buffer.push(c);

                    let chars: Vec<char> = buffer.chars().collect();
                    let mut instruction_buffer = 0;
                    for i in 0..chars.len() {
                        if chars[i] == ONE {
                            instruction_buffer += 0b100 >> i;
                        }
                    }

                    match instruction_buffer {
                        0b000 => {result.push(Instruction::PointerRight)},
                        0b001 => {result.push(Instruction::PointerLeft)},
                        0b010 => {result.push(Instruction::Increase)},
                        0b011 => {result.push(Instruction::Decrease)},
                        0b100 => {result.push(Instruction::Print)},
                        0b101 => {result.push(Instruction::Input)},
                        0b110 => {result.push(Instruction::LoopBegin)},
                        0b111 => {result.push(Instruction::LoopEnd)},
                        _ => panic!("Unexpected input.")
                    }

                    counter = 0;
                    buffer = "".to_string();
                },
                _ => counter = 0,
            }
        } else {
            buffer = "".to_string();
            counter = 0;
        }
    }

    return result;
}

fn main() {
    let file_content: String = fs::read_to_string("README.md").unwrap();
    println!("{:?}", file_to_instruction(String::from(file_content)));
}
