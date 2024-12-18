use aoc::prelude::*;
use core::str;

pub const PROG_LEN: usize = 16;
const A: usize = 4;
const B: usize = 5;
const C: usize = 6;

pub fn parse(input: &str) -> (usize, [u8; PROG_LEN]) {
    let mut crs = input.as_bytes().as_ptr();
    crs.skip("Register A: ".len());

    let a: usize = crs.parse_uint_n_digits(crs.find(b'\n'));
    crs.skip("\nRegister B: 0\nRegister C: 0\n\nProgram: ".len());

    let program = crs.parse_n_uints::<u8, PROG_LEN, 1>(1);

    (a, program)
}

pub fn p1<'o>(a: usize, prog: &[u8; PROG_LEN], out: &'o mut [u8; PROG_LEN + 1]) -> &'o str {
    let mut reg = [0, 1, 2, 3, a, 0, 0];
    let mut ip = 0;
    for i in 0..(a as f32).log(8.0).ceil() as usize {
        for _ in 0..(prog.len() >> 1) - 1 {
            let op = Opcode::from(prog[ip]);
            let operand = prog[ip + 1] as usize;

            match op {
                Opcode::ADV => reg[A] = reg[A] >> reg[operand],
                Opcode::BXL => reg[B] ^= operand,
                Opcode::BST => reg[B] = reg[operand] & 0b111,
                Opcode::BXC => reg[B] ^= reg[C],
                Opcode::OUT => out[i << 1] = (reg[operand] & 0b111) as u8 + b'0',
                Opcode::CDV => reg[C] = reg[A] >> reg[operand],
                _ => unreachable!(),
            }
            ip = (ip + 2) % (prog.len() - 2);
        }
    }

    unsafe { std::str::from_utf8_unchecked(out) }
}

pub fn p2(prog: &[u8; PROG_LEN]) -> usize {
    const MIN_A: usize = 8usize.pow((PROG_LEN - 1) as u32);
    // const MAX_A: usize = 8usize.pow(PROGRAM_LEN as u32) - 1;

    let mut a = MIN_A;
    for digit in (0..prog.len()).rev() {
        let step = 1 << digit * 3;

        while !check_digits(a, digit, prog) {
            a += step;
        }
    }

    a
}

fn check_digits(mut a: usize, digit: usize, prog: &[u8; PROG_LEN]) -> bool {
    a = a >> digit * 3;
    for i in digit..prog.len() {
        let val = ((((a & 0b111) ^ 7) ^ (a >> (((a & 0b111) ^ 7)))) ^ 4) & 0b111;
        if prog[i] as usize != val {
            return false;
        }
        a >>= 3;
    }

    true
}

#[derive(Debug)]
#[repr(u8)]
enum Opcode {
    ADV = 0, // A = floor(A/pow(2, <COMBO>)); IP += 2
    BXL = 1, // B = B ^ <LITERAL>; IP += 2
    BST = 2, // B = <COMBO> % 8; IP += 2
    JNZ = 3, // if A == 0 { IP += 2 } else { IP += <LITERAL> }
    BXC = 4, // B = B ^ C; IP += 2
    OUT = 5, // output(<COMBO> % 8); IP += 2
    BDV = 6, // B = floor(A/pow(2, <COMBO>)); IP += 2
    CDV = 7, // C = floor(A/pow(2, <COMBO>)); IP += 2
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute::<u8, Opcode>(value) }
    }
}

// B = A % 8
// B = B ^ 7
// C = A / 2 ** B
// B = B ^ C
// B = B ^ 4
// => B % 8
// A = A / 8

// => ((((A % 8) ^ 7) ^ (A / 2 ** ((A % 8) ^ 7))) ^ 4) % 8
// A = A / 8

// (((($A mod 8) bit-xor 7) bit-xor ($A / 2 ** (($A mod 8) bit-xor 7) | math floor)) bit-xor 4) mod 8

// => (((7..0) ^ (A / 2 ** (7..0))) ^ 4) % 8
