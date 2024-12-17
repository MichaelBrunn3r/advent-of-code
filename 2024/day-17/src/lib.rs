use core::str;

use aoc::prelude::*;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

const PROGRAM_LEN: usize = 16;
const A: usize = 4;
const B: usize = 5;
const C: usize = 6;

pub fn parse(input: &str) -> (usize, [u8; PROGRAM_LEN]) {
    let mut crs = input.as_bytes().as_ptr();
    crs.skip("Register A: ".len());

    let a: usize = crs.parse_uint_n_digits(crs.find(b'\n'));
    crs.skip("\nRegister B: 0\nRegister C: 0\n\nProgram: ".len());

    let program = crs.parse_n_uints::<u8, PROGRAM_LEN, 1>(1);

    (a, program)
}

pub fn p1(a: usize, prog: &[u8; PROGRAM_LEN]) -> String {
    let mut output: Vec<u8> = Vec::new();
    let mut reg = [0, 1, 2, 3, a, 0, 0];
    
    let mut ip = 0;
    while ip < prog.len() {
        let op = unsafe { std::mem::transmute::<u8, Opcode>(prog[ip]) };
        let operand = prog[ip+1];

        match op {
            Opcode::ADV => reg[A] = reg[A] / (1 << reg[operand as usize]),
            Opcode::BXL => reg[B] ^= operand as usize,
            Opcode::BST => reg[B] = reg[operand as usize] % 8,
            Opcode::JNZ => if reg[A] != 0 {
                ip = operand as usize;
                continue;
            }
            Opcode::BXC => reg[B] ^= reg[C],
            Opcode::OUT => output.push((reg[operand as usize] % 8) as u8),
            Opcode::BDV => reg[B] = reg[A] / (1 << reg[operand as usize]),
            Opcode::CDV => reg[C] = reg[A] / (1 << reg[operand as usize]),
        }

        ip += 2;
    }

    let mut result = vec![b','; output.len() * 2 - 1];
    output.iter().enumerate().for_each(|(i, &x)| result[i << 1] = (x as u8) + b'0');
    unsafe{String::from_utf8_unchecked(result)}
}

pub fn p2(prog: &[u8; PROGRAM_LEN]) -> usize {
    0
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
