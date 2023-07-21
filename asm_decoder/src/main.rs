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
            let reg = byte1 & 0b00000111;
            let data = if w == 1 {
                let data = buf[byte_idx + 1] as u16 | ((buf[byte_idx + 2] as u16) << 8);
                byte_idx += 2;
                data.to_string()
            } else {
                let data = buf[byte_idx + 1];
                byte_idx += 1;
                data.to_string()
            };
            print_op(
                decode_opcode(immediate_opcode),
                decode_reg(reg, w),
                &data,
                false,
                false,
                None,
            );
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
                        disp_lo = Some((buf[byte_idx + 2]).to_string());
                        byte_idx += 1;
                    }
                    print_op(
                        decoded_opcode,
                        decode_reg(reg, w),
                        decode_memory(rm),
                        d == 0,
                        true,
                        disp_lo.as_deref(),
                    );
                }
                // memory 8bit displacement
                0b01 => {
                    print_op(
                        decoded_opcode,
                        decode_reg(reg, w),
                        decode_memory(rm),
                        d == 0,
                        true,
                        Some(&(buf[byte_idx + 2]).to_string()),
                    );
                    byte_idx += 1;
                }
                // memory 16bit displacement
                0b10 => {
                    print_op(
                        decoded_opcode,
                        decode_reg(reg, w),
                        decode_memory(rm),
                        d == 0,
                        true,
                        Some(
                            &(buf[byte_idx + 2] as u16 | ((buf[byte_idx + 3] as u16) << 8))
                                .to_string(),
                        ),
                    );
                    byte_idx += 2;
                }
                // reg reg
                0b11 => {
                    print_op(
                        decoded_opcode,
                        decode_reg(reg, w),
                        decode_reg(rm, w),
                        d == 0,
                        false,
                        None,
                    );
                }
                _ => panic!("Illegal mod encoding"),
            }
            byte_idx += 1;
        }
        byte_idx += 1;
    }
}

fn print_op<'a>(
    opcode: &'a str,
    dest: &'a str,
    source: &'a str,
    swap: bool,
    memory: bool,
    disp: Option<&'a str>,
) {
    if memory {
        if let Some(disp) = disp {
            if swap {
                println!("{} [{} + {}], {}", opcode, source, disp, dest);
            } else {
                println!("{} {}, [{} + {}]", opcode, dest, source, disp);
            }
        } else if swap {
            println!("{} [{}], {}", opcode, source, dest);
        } else {
            println!("{} {}, [{}]", opcode, dest, source);
        }
    } else if swap {
        println!("{} {}, {}", opcode, source, dest);
    } else {
        println!("{} {}, {}", opcode, dest, source);
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

fn decode_memory(rm: u8) -> &'static str {
    match rm {
        0b000 => "bx + si",
        0b001 => "bx + di",
        0b010 => "bp + si",
        0b011 => "bp + di",
        0b100 => "si",
        0b101 => "di",
        0b110 => "bp",
        0b111 => "bx",
        _ => panic!("R/M wrong encoding: {:b}", rm),
    }
}
