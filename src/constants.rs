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