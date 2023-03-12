use clap::{arg, value_parser, Command};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

lazy_static! {
    pub static ref W0_REGISTERS: HashMap<u8, String> = {
        let w0_registers: HashMap<u8, String> = [
            (0b000, "al".to_string()),
            (0b001, "cl".to_string()),
            (0b010, "dl".to_string()),
            (0b011, "bl".to_string()),
            (0b100, "ah".to_string()),
            (0b101, "ch".to_string()),
            (0b110, "dh".to_string()),
            (0b111, "bh".to_string()),
        ]
        .iter()
        .cloned()
        .collect();
        w0_registers
    };
}

lazy_static! {
    pub static ref W1_REGISTERS: HashMap<u8, String> = {
        let w1_registers: HashMap<u8, String> = [
            (0b000, "ax".to_string()),
            (0b001, "cx".to_string()),
            (0b010, "dx".to_string()),
            (0b011, "bx".to_string()),
            (0b100, "sp".to_string()),
            (0b101, "bp".to_string()),
            (0b110, "si".to_string()),
            (0b111, "di".to_string()),
        ]
        .iter()
        .cloned()
        .collect();
        w1_registers
    };
}

fn main() {
    let matches = Command::new("Intel 8086")
        .version("0.1.0")
        .author("4nc3str4l <murielmaths@gmail.com>")
        .about("An intel 8086 instruction set decoder")
        .arg(
            arg!(--f <FILE>)
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let target_path = matches.get_one::<PathBuf>("f").expect("required");

    let mut target_file = File::open(target_path).unwrap();

    println!("Target File: {:?}", target_path);
    let mut buffer = Vec::new();
    target_file.read_to_end(&mut buffer).unwrap();
    println!("{}", decode_buffer(&buffer));
}

fn decode_buffer(buffer: &[u8]) -> String {
    assert!(!buffer.is_empty(), "Empty files are not allowed");
    assert!(buffer.len() % 2 == 0, "Number of bytes must be even");

    let mut result = String::from("bits 16\n");
    for chunk in buffer.chunks(2).map(|chunk| (chunk[0], chunk[1])) {
        let instruction = decode_instruction(chunk.0);
        let w = get_w_value(chunk.0);
        let _d = get_d_value(chunk.0);
        let _mod = get_mod(chunk.1);
        let first_register = decode_first_register(chunk.1, w);
        let second_register = decode_second_register(chunk.1, w);
        result += &format!("\n{} {}, {}", instruction, first_register, second_register);
    }

    result
}

fn decode_instruction(instruction: u8) -> String {
    match instruction >> 2 == 0b100010 {
        true => "mov".to_owned(),
        false => String::from("UNKNOWN"),
    }
}

fn get_d_value(data: u8) -> bool {
    ((data & 0b0000_0010) >> 1) > 0
}

fn get_w_value(data: u8) -> bool {
    (data & 0b0000_0001) > 0
}

fn get_mod(data: u8) -> u8 {
    data >> 6
}

fn decode_first_register(data: u8, w: bool) -> String {
    byte_to_register(data & 0b00000111, w)
}

fn decode_second_register(data: u8, w: bool) -> String {
    let remove_mod = data & 0b00111111;
    byte_to_register(remove_mod >> 3, w)
}

fn byte_to_register(byte: u8, w: bool) -> String {
    match w {
        false => match W0_REGISTERS.get(&byte) {
            Some(register) => register.to_owned(),
            None => panic!("Undefined register for {}", byte),
        },
        true => match W1_REGISTERS.get(&byte) {
            Some(register) => register.to_owned(),
            None => panic!("Undefined register for {}", byte),
        },
    }
}

#[test]
fn test_decode_mov() {
    assert_eq!(decode_buffer(&vec![0x89, 0xd9]), "bits 16\n\nmov cx, bx")
}
