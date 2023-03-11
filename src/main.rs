use clap::builder::Str;
use clap::{arg, value_parser, Command};
use std::path::PathBuf;
use std::io::{Read};
use std::fs::File;
use lazy_static::lazy_static;
use std::collections::HashMap;

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
        .arg(arg!(--f <FILE>).required(true).value_parser(value_parser!(PathBuf)))
        .get_matches();

    let target_path = matches.get_one::<PathBuf>("f").expect("required");

    let mut target_file = File::open(target_path).unwrap();
    
    let mut buffer = Vec::new();
    target_file.read_to_end(&mut buffer).unwrap();

    println!("Target File: {:?}", target_path);

    println!("bits 16\n");
    for n in 0..buffer.len() / 2 {
        let offset = n*2;
        let instruction = decode_instruction(buffer[offset]);
        let d = get_d_value(buffer[offset]);
        let w = get_w_value(buffer[offset]);
        let mode = get_mod(buffer[offset + 1]);
        let first_register = decode_first_register(buffer[offset + 1], w);
        let second_register = decode_second_register(buffer[offset + 1], w);
        println!("{} {}, {}", instruction, first_register, second_register);
    }
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
    (data & 0b1100_0000) >> 6
}

fn decode_first_register(data: u8, w: bool) -> String {
    let reg2 = data & 0b00000111;
    match w {
        false => W0_REGISTERS[&reg2].to_owned(),
        true => W1_REGISTERS[&reg2].to_owned(),
    }
}

fn decode_second_register(data: u8, w: bool) -> String {
    let remove_mod = data & 0b00111111;
    let reg1 = remove_mod >> 3;
    match w {
        false => W0_REGISTERS[&reg1].to_owned(),
        true => W1_REGISTERS[&reg1].to_owned(),
    }
}



