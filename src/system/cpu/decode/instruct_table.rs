use crate::system::cpu::{Cpu, decode::Instruction};

pub fn load_instruction_table() -> [Instruction; 256] {
    let mut table = [Instruction::default(); 256];

    // --- LD r8, r8 ---
    // LD B, r8
    table[0x40] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD B,B");
    table[0x41] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD B,C");
    table[0x42] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD B,D");
    table[0x43] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD B,E");
    table[0x44] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD B,H");
    table[0x45] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD B,L");
    table[0x46] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD B,[HL]");
    table[0x47] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD B,A");
    // LD C, r8
    table[0x48] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD C,B");
    table[0x49] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD C,C");
    table[0x4A] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD C,D");
    table[0x4B] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD C,E");
    table[0x4C] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD C,H");
    table[0x4D] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD C,L");
    table[0x4E] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD C,[HL]");
    table[0x4F] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD C,A");
    // LD D, r8
    table[0x50] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD D,B");
    table[0x51] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD D,C");
    table[0x52] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD D,D");
    table[0x53] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD D,E");
    table[0x54] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD D,H");
    table[0x55] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD D,L");
    table[0x56] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD D,[HL]");
    table[0x57] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD D,A");
    // LD E, r8
    table[0x58] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD E,B");
    table[0x59] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD E,C");
    table[0x5A] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD E,D");
    table[0x5B] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD E,E");
    table[0x5C] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD E,H");
    table[0x5D] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD E,L");
    table[0x5E] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD E,[HL]");
    table[0x5F] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD E,A");
    // LD H, r8
    table[0x60] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD H,B");
    table[0x61] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD H,C");
    table[0x62] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD H,D");
    table[0x63] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD H,E");
    table[0x64] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD H,H");
    table[0x65] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD H,L");
    table[0x66] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD H,[HL]");
    table[0x67] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD H,A");
    // LD L, r8
    table[0x68] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD L,B");
    table[0x69] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD L,C");
    table[0x6A] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD L,D");
    table[0x6B] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD L,E");
    table[0x6C] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD L,H");
    table[0x6D] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD L,L");
    table[0x6E] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD L,[HL]");
    table[0x6F] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD L,A");
    // LD [HL], r8
    table[0x70] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD [HL],B");
    table[0x71] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD [HL],C");
    table[0x72] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD [HL],D");
    table[0x73] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD [HL],E");
    table[0x74] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD [HL],H");
    table[0x75] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD [HL],L");
    table[0x77] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD [HL],A");
    // LD A, r8
    table[0x78] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD A,B");
    table[0x79] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD A,C");
    table[0x7A] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD A,D");
    table[0x7B] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD A,E");
    table[0x7C] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD A,H");
    table[0x7D] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD A,L");
    table[0x7E] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD A,[HL]");
    table[0x7F] = Instruction::new(4, 1, Cpu::ld_r8_r8, "LD A,A");

    // --- LD r8, n8 ---
    table[0x06] = Instruction::new(8, 2, Cpu::ld_r8_n8, "LD B,n8");
    table[0x0E] = Instruction::new(8, 2, Cpu::ld_r8_n8, "LD C,n8");
    table[0x16] = Instruction::new(8, 2, Cpu::ld_r8_n8, "LD D,n8");
    table[0x1E] = Instruction::new(8, 2, Cpu::ld_r8_n8, "LD E,n8");
    table[0x26] = Instruction::new(8, 2, Cpu::ld_r8_n8, "LD H,n8");
    table[0x2E] = Instruction::new(8, 2, Cpu::ld_r8_n8, "LD L,n8");
    table[0x36] = Instruction::new(12, 2, Cpu::ld_r8_n8, "LD [HL],n8");
    table[0x3E] = Instruction::new(8, 2, Cpu::ld_r8_n8, "LD A,n8");

    // --- LD r16, n16
    table[0x01] = Instruction::new(12, 3, Cpu::ld_r16_n16, "LD BC,n16");
    table[0x11] = Instruction::new(12, 3, Cpu::ld_r16_n16, "LD DE,n16");
    table[0x21] = Instruction::new(12, 3, Cpu::ld_r16_n16, "LD HL,n16");
    table[0x31] = Instruction::new(12, 3, Cpu::ld_r16_n16, "LD SP,n16");

    table
}
