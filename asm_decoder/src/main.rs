use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("./single").expect("File to be present");
    let metadata = file.metadata().unwrap();
    let capacity = metadata.len() as usize;
    let mut buf = Vec::with_capacity(capacity);
    file.read_to_end(&mut buf).expect("Read working");
    assert_eq!(buf.len(), 2);
    let byte1 = buf[0];
    let opcode = (byte1 & 0b11111100) >> 2;
    let d = (byte1 & 0b00000010) >> 1;
    let w = byte1 & 0b00000001;
    //println!("opcode: {:b}, d: {:b}, w: {:b}", opcode, d, w);
    let byte2 = buf[1];
    let mod_field = (byte2 & 0b11000000) >> 6;
    let reg = (byte2 & 0b00111000) >> 3;
    let rw = byte2 & 0b00000111;
    //println!("mod: {:b}, reg: {:b}, rw: {:b}", mod_field, reg, rw);
    println!("bits 16");
    println!(
        "{} {}, {}",
        decode_opcode(opcode),
        decode_reg(rw, w),
        decode_reg(reg, w),
    );
}

fn decode_opcode(opcode: u8) -> &'static str {
    match opcode {
        0b100010 => "mov",
        _ => panic!("Invalid encoding"),
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
        _ => panic!("Invalid encoding"),
    }
}
