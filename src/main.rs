use std::fs;
use std::io::Read;

const ZERO: char = ' ';
const ONE: char = '\t';
const IGNORE_NEWLINES: bool = false;
const TAPE_LENGTH: usize = 1024;

const POINTER_RIGHT_COMBINATION: u8 = 0b000;
const POINTER_LEFT_COMBINATION: u8 = 0b001;
const INCREASE_COMBINATION: u8 = 0b010;
const DECREASE_COMBINATION: u8 = 0b011;
const PRINT_COMBINATION: u8 = 0b100;
const INPUT_COMBINATION: u8 = 0b101;
const LOOP_BEGIN_COMBINATION: u8 = 0b110;
const LOOP_END_COMBINATION: u8 = 0b111;

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
                }
                3 => {
                    buffer.push(c);

                    let chars: Vec<char> = buffer.chars().collect();
                    let mut instruction_buffer: u8 = 0;
                    for i in 0..chars.len() {
                        if chars[i] == ONE {
                            instruction_buffer += 0b100 >> i;
                        }
                    }

                    match instruction_buffer {
                        POINTER_RIGHT_COMBINATION => result.push(Instruction::PointerRight),
                        POINTER_LEFT_COMBINATION => result.push(Instruction::PointerLeft),
                        INCREASE_COMBINATION => result.push(Instruction::Increase),
                        DECREASE_COMBINATION => result.push(Instruction::Decrease),
                        PRINT_COMBINATION => result.push(Instruction::Print),
                        INPUT_COMBINATION => result.push(Instruction::Input),
                        LOOP_BEGIN_COMBINATION => result.push(Instruction::LoopBegin),
                        LOOP_END_COMBINATION => result.push(Instruction::LoopEnd),
                        _ => panic!("Unexpected input."),
                    }

                    counter = 0;
                    buffer = "".to_string();
                }
                _ => counter = 0,
            }
        } else {
            buffer = "".to_string();
            counter = 0;
        }
    }

    return result;
}

fn run(instructions: Vec<Instruction>) {
    let mut tape: Vec<u8> = vec![0; TAPE_LENGTH];
    let mut data_pointer: usize = 0;
    let mut instruction_pointer: usize = 0;
    while instructions.len() > instruction_pointer {
        match instructions[instruction_pointer] {
            Instruction::PointerRight => {
                if data_pointer < TAPE_LENGTH - 1 {
                    data_pointer += 1;
                }
            }
            Instruction::PointerLeft => {
                if data_pointer != 0 {
                    data_pointer -= 1;
                }
            }
            Instruction::Increase => {
                tape[data_pointer] = tape[data_pointer].wrapping_add(1);
            }
            Instruction::Decrease => {
                tape[data_pointer] = tape[data_pointer].wrapping_sub(1);
            }
            Instruction::Print => {
                print!("{}", tape[data_pointer] as char);
            }
            Instruction::Input => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin()
                    .read_exact(&mut input)
                    .expect("failed to read stdin");
                tape[data_pointer] = input[0];
            }
            Instruction::LoopBegin => {
                if tape[data_pointer] == 0 {
                    let mut nesting: usize = 1;
                    while nesting != 0 {
                        if instruction_pointer != instructions.len() - 1 {
                            instruction_pointer += 1;
                        } else {
                            panic!("No matching ']' found");
                        }
                        if matches!(instructions[instruction_pointer], Instruction::LoopEnd) {
                            nesting -= 1;
                        } else if matches!(
                            instructions[instruction_pointer],
                            Instruction::LoopBegin
                        ) {
                            nesting += 1;
                        }
                    }
                }
            }
            Instruction::LoopEnd => {
                if tape[data_pointer] != 0 {
                    let mut nesting: usize = 1;
                    while nesting != 0 {
                        if instruction_pointer != 0 {
                            instruction_pointer -= 1;
                        } else {
                            panic!("No matching '[' found");
                        }
                        if matches!(instructions[instruction_pointer], Instruction::LoopBegin) {
                            nesting -= 1;
                        } else if matches!(instructions[instruction_pointer], Instruction::LoopEnd)
                        {
                            nesting += 1;
                        }
                    }
                }
            }
        }
        instruction_pointer += 1;
    }
}

fn main() {
    let filepath = std::env::args().nth(1).expect("No path given.");
    let file_content: String = fs::read_to_string(filepath).unwrap();
    run(file_to_instruction(String::from(file_content)));
}
