use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("./complex_mov").expect("File to be present");
    let metadata = file.metadata().unwrap();
    let capacity = metadata.len() as usize;
    let mut buf = Vec::with_capacity(capacity);
    file.read_to_end(&mut buf).expect("Read working");
    let mut byte_idx = 0;
    println!("bits 16");
    while byte_idx < capacity {
        let byte1 = buf[byte_idx];
        let immediate_opcode = (byte1 & 0b11110000) >> 4;
        let others_opcode = (byte1 & 0b11111100) >> 2;
        if immediate_opcode == 0b1011 {
            let w = (byte1 & 0b00001000) >> 3;
            let reg = (byte1 & 0b00000111);
            if w == 1 {
                println!(
                    "{:b}, {:b}",
                    (buf[byte_idx + 2] as u16) << 8,
                    (buf[byte_idx + 1] as u16)
                );
                let data = buf[byte_idx + 1] as u16 | ((buf[byte_idx + 2] as u16) << 8);
                println!(
                    "{} {}, {}",
                    decode_opcode(immediate_opcode),
                    decode_reg(reg, w),
                    data
                );
                byte_idx += 2;
            } else {
                let data = buf[byte_idx + 1];
                println!(
                    "{} {}, {}",
                    decode_opcode(immediate_opcode),
                    decode_reg(reg, w),
                    data
                );
                byte_idx += 1;
            }
        } else {
            let decoded_opcode = decode_opcode(others_opcode);
            let d = (byte1 & 0b00000010) >> 1;
            let w = byte1 & 0b00000001;
            let byte2 = buf[byte_idx + 1];
            let mod_field = (byte2 & 0b11000000) >> 6;
            let reg = (byte2 & 0b00111000) >> 3;
            let rm = byte2 & 0b00000111;
            match mod_field {
                // memory no displacement
                0b00 => {
                    let mut disp_lo = None;
                    if rm == 0b110 {
                        disp_lo = Some(0);
                        byte_idx += 1;
                    }
                    println!(
                        "{} {}, {}",
                        decoded_opcode,
                        decode_memory(rm, w, disp_lo, None),
                        decode_reg(reg, w),
                    );
                }
                // memory 8bit displacement
                0b01 => {
                    let disp_lo = 0;
                    let disp_hi = 0;
                    byte_idx += 2;
                    println!(
                        "{} {}, {}",
                        decoded_opcode,
                        decode_memory(rm, w, Some(disp_lo), Some(disp_hi)),
                        decode_reg(reg, w),
                    );
                }
                // memory 16bit displacement
                0b10 => {
                    println!(
                        "{} {}, {}",
                        decoded_opcode,
                        decode_memory(rm, w, None, None),
                        decode_reg(reg, w),
                    );
                }
                // reg reg
                0b11 => {
                    println!(
                        "{} {}, {}",
                        decoded_opcode,
                        decode_reg(rm, w),
                        decode_reg(reg, w),
                    );
                }
                _ => panic!("Illegal mod encodinc"),
            }
            byte_idx += 1;
        }
        byte_idx += 1;
    }
}

fn decode_opcode(opcode: u8) -> &'static str {
    match opcode {
        0b100010 => "mov",
        0b1011 => "mov",
        _ => panic!("Invalid encoding: {:b}", opcode),
    }
}

fn decode_reg(reg: u8, w: u8) -> &'static str {
    match reg {
        0b000 => {
            if w == 0 {
                "al"
            } else {
                "ax"
            }
        }
        0b001 => {
            if w == 0 {
                "cl"
            } else {
                "cx"
            }
        }
        0b010 => {
            if w == 0 {
                "dl"
            } else {
                "dx"
            }
        }
        0b011 => {
            if w == 0 {
                "bl"
            } else {
                "bx"
            }
        }
        0b100 => {
            if w == 0 {
                "ah"
            } else {
                "sp"
            }
        }
        0b101 => {
            if w == 0 {
                "ch"
            } else {
                "bp"
            }
        }
        0b110 => {
            if w == 0 {
                "dh"
            } else {
                "si"
            }
        }
        0b111 => {
            if w == 0 {
                "bh"
            } else {
                "di"
            }
        }
        _ => panic!("Invalid encoding {:b}", reg),
    }
}

fn decode_memory(rm: u8, w: u8, disp_lo: Option<u8>, disp_hi: Option<u8>) -> &'static str {
    "memory"
}
