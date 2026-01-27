use crate::system::cpu::{Cpu, decode::Instruction};

pub const fn load_instruction_table() -> [Instruction; 256] {
    let unknown_instruction = Instruction::new(0, 1, Cpu::unknown_instr, "Unknown Instruction!");
    let mut table = [unknown_instruction; 256];

    // --- NOP ---
    table[0x00] = Instruction::new(4, 1, Cpu::nop, "NOP");

    // --- HALT ---
    table[0x76] = Instruction::new(4, 1, Cpu::halt, "HALT");

    // --- STOP ---
    table[0x10] = Instruction::new(4, 2, Cpu::stop, "STOP");

    // --- DI ---
    table[0xF3] = Instruction::new(4, 1, Cpu::di, "DI");

    // --- EI ---
    table[0xFB] = Instruction::new(4, 1, Cpu::ie, "IE");

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
    table[0x36] = Instruction::new(8, 2, Cpu::ld_r8_n8, "LD [HL],n8");
    table[0x3E] = Instruction::new(8, 2, Cpu::ld_r8_n8, "LD A,n8");

    // --- LD r16, n16 ---
    table[0x01] = Instruction::new(12, 3, Cpu::ld_r16_n16, "LD BC,n16");
    table[0x11] = Instruction::new(12, 3, Cpu::ld_r16_n16, "LD DE,n16");
    table[0x21] = Instruction::new(12, 3, Cpu::ld_r16_n16, "LD HL,n16");
    table[0x31] = Instruction::new(12, 3, Cpu::ld_r16_n16, "LD SP,n16");

    // --- LD [r16], A ---
    table[0x02] = Instruction::new(8, 1, Cpu::ld_r16mem_a, "LD [BC],A");
    table[0x12] = Instruction::new(8, 1, Cpu::ld_r16mem_a, "LD [DE],A");
    table[0x22] = Instruction::new(8, 1, Cpu::ld_r16mem_a, "LD [HL+],A");
    table[0x32] = Instruction::new(8, 1, Cpu::ld_r16mem_a, "LD [HL-],A");

    // --- LD [n16], A ---
    table[0xEA] = Instruction::new(16, 3, Cpu::ld_n16mem_a, "LD [n16],A");

    // --- LDH [n8], A ---
    table[0xE0] = Instruction::new(12, 2, Cpu::ldh_n8mem_a, "LDH [n8],A");

    // --- LDH [C], A ---
    table[0xE2] = Instruction::new(8, 1, Cpu::ldh_cmem_a, "LDH [C],A");

    // --- LD A, [r16] ---
    table[0x0A] = Instruction::new(8, 1, Cpu::ld_a_r16mem, "LD A,[BC]");
    table[0x1A] = Instruction::new(8, 1, Cpu::ld_a_r16mem, "LD A,[DE]");
    table[0x2A] = Instruction::new(8, 1, Cpu::ld_a_r16mem, "LD A,[HL+]");
    table[0x3A] = Instruction::new(8, 1, Cpu::ld_a_r16mem, "LD A,[HL-]");

    // --- LD A, [n16] ---
    table[0xFA] = Instruction::new(16, 3, Cpu::ld_a_n16mem, "LD A,[n16]");

    // --- LDH A, [n8] ---
    table[0xF0] = Instruction::new(12, 2, Cpu::ld_a_n8mem, "LDH A,[n8]");

    // --- LDH A, [C] ---
    table[0xF2] = Instruction::new(8, 1, Cpu::ld_a_cmem, "LDH A,[C]");

    // --- LD [n16], SP ---
    table[0x08] = Instruction::new(20, 3, Cpu::ld_n16mem_sp, "LD [n16],SP");

    // --- LD HL, SP + e8 ---
    table[0xF8] = Instruction::new(12, 2, Cpu::ld_hl_sp_e8, "LD HL,SP+e8");

    // --- LD SP, HL ---
    table[0xF9] = Instruction::new(8, 1, Cpu::ld_sp_hl, "LD SP,HL");

    // --- POP r16 ---
    table[0xC1] = Instruction::new(12, 1, Cpu::pop_r16stk, "POP BC");
    table[0xD1] = Instruction::new(12, 1, Cpu::pop_r16stk, "POP DE");
    table[0xE1] = Instruction::new(12, 1, Cpu::pop_r16stk, "POP HL");
    table[0xF1] = Instruction::new(12, 1, Cpu::pop_r16stk, "POP AF");

    // --- PUSH r16 ---
    table[0xC5] = Instruction::new(16, 1, Cpu::push_r16stk, "PUSH BC");
    table[0xD5] = Instruction::new(16, 1, Cpu::push_r16stk, "PUSH DE");
    table[0xE5] = Instruction::new(16, 1, Cpu::push_r16stk, "PUSH HL");
    table[0xF5] = Instruction::new(16, 1, Cpu::push_r16stk, "PUSH AF");

    // --- JP n16 ---
    table[0xC3] = Instruction::new(16, 3, Cpu::jp_n16, "JP n16");

    // --- JP cc, n16 ---
    table[0xCA] = Instruction::new(12, 3, Cpu::jp_z_n16, "JP Z,n16");
    table[0xC2] = Instruction::new(12, 3, Cpu::jp_nz_n16, "JP NZ,n16");
    table[0xDA] = Instruction::new(12, 3, Cpu::jp_c_n16, "JP C,n16");
    table[0xD2] = Instruction::new(12, 3, Cpu::jp_nc_n16, "JP NC,n16");

    // --- JR e8 ---
    table[0x18] = Instruction::new(12, 2, Cpu::jr_e8, "JR e8");

    // --- JR cc, e8 ---
    table[0x28] = Instruction::new(8, 2, Cpu::jr_z_e8, "JR Z,e8");
    table[0x20] = Instruction::new(8, 2, Cpu::jr_nz_e8, "JR NZ,e8");
    table[0x38] = Instruction::new(8, 2, Cpu::jr_c_e8, "JR C,e8");
    table[0x30] = Instruction::new(8, 2, Cpu::jr_nc_e8, "JR NC,e8");

    // --- JR HL ---
    table[0xE9] = Instruction::new(4, 1, Cpu::jr_hl, "JR HL");

    // --- CALL n16 ---
    table[0xCD] = Instruction::new(24, 3, Cpu::call_n16, "CALL n16");

    // --- CALL cc, n16 ---
    table[0xCC] = Instruction::new(12, 3, Cpu::call_z_n16, "CALL Z,n16");
    table[0xC4] = Instruction::new(12, 3, Cpu::call_nz_n16, "CALL NZ,n16");
    table[0xDC] = Instruction::new(12, 3, Cpu::call_c_n16, "CALL C,n16");
    table[0xD4] = Instruction::new(12, 3, Cpu::call_nc_n16, "CALL NC,n16");

    // --- RET ---
    table[0xC9] = Instruction::new(16, 1, Cpu::ret, "RET");

    // --- RET cc ---
    table[0xC8] = Instruction::new(8, 1, Cpu::ret_z, "RET Z");
    table[0xC0] = Instruction::new(8, 1, Cpu::ret_nz, "RET NZ");
    table[0xD8] = Instruction::new(8, 1, Cpu::ret_c, "RET C");
    table[0xD0] = Instruction::new(8, 1, Cpu::ret_nc, "RET NC");

    // --- RETI ---
    table[0xD9] = Instruction::new(16, 1, Cpu::reti, "RETI");

    // --- RST vec ---
    table[0xC7] = Instruction::new(16, 1, Cpu::rst_vec, "RST $00");
    table[0xCF] = Instruction::new(16, 1, Cpu::rst_vec, "RST $08");
    table[0xD7] = Instruction::new(16, 1, Cpu::rst_vec, "RST $10");
    table[0xDF] = Instruction::new(16, 1, Cpu::rst_vec, "RST $18");
    table[0xE7] = Instruction::new(16, 1, Cpu::rst_vec, "RST $20");
    table[0xEF] = Instruction::new(16, 1, Cpu::rst_vec, "RST $28");
    table[0xF7] = Instruction::new(16, 1, Cpu::rst_vec, "RST $30");
    table[0xFF] = Instruction::new(16, 1, Cpu::rst_vec, "RST $38");

    // --- ADD A, r8 ---
    table[0x80] = Instruction::new(4, 1, Cpu::add_a_r8, "ADD A,B");
    table[0x81] = Instruction::new(4, 1, Cpu::add_a_r8, "ADD A,C");
    table[0x82] = Instruction::new(4, 1, Cpu::add_a_r8, "ADD A,D");
    table[0x83] = Instruction::new(4, 1, Cpu::add_a_r8, "ADD A,E");
    table[0x84] = Instruction::new(4, 1, Cpu::add_a_r8, "ADD A,H");
    table[0x85] = Instruction::new(4, 1, Cpu::add_a_r8, "ADD A,L");
    table[0x86] = Instruction::new(4, 1, Cpu::add_a_r8, "ADD A,[HL]");
    table[0x87] = Instruction::new(4, 1, Cpu::add_a_r8, "ADD A,A");

    // --- ADD A, n8 ---
    table[0xC6] = Instruction::new(8, 2, Cpu::add_a_n8, "ADD A,n8");

    // --- ADC A, r8 ---
    table[0x88] = Instruction::new(4, 1, Cpu::adc_a_r8, "ADC A,B");
    table[0x89] = Instruction::new(4, 1, Cpu::adc_a_r8, "ADC A,C");
    table[0x8a] = Instruction::new(4, 1, Cpu::adc_a_r8, "ADC A,D");
    table[0x8b] = Instruction::new(4, 1, Cpu::adc_a_r8, "ADC A,E");
    table[0x8c] = Instruction::new(4, 1, Cpu::adc_a_r8, "ADC A,H");
    table[0x8d] = Instruction::new(4, 1, Cpu::adc_a_r8, "ADC A,L");
    table[0x8e] = Instruction::new(4, 1, Cpu::adc_a_r8, "ADC A,[HL]");
    table[0x8f] = Instruction::new(4, 1, Cpu::adc_a_r8, "ADC A,A");

    // --- ADC A, n8 ---
    table[0xCE] = Instruction::new(8, 2, Cpu::adc_a_n8, "ADC A,n8");

    // --- SUB A, r8 ---
    table[0x90] = Instruction::new(4, 1, Cpu::sub_a_r8, "SUB A,B");
    table[0x91] = Instruction::new(4, 1, Cpu::sub_a_r8, "SUB A,C");
    table[0x92] = Instruction::new(4, 1, Cpu::sub_a_r8, "SUB A,D");
    table[0x93] = Instruction::new(4, 1, Cpu::sub_a_r8, "SUB A,E");
    table[0x94] = Instruction::new(4, 1, Cpu::sub_a_r8, "SUB A,H");
    table[0x95] = Instruction::new(4, 1, Cpu::sub_a_r8, "SUB A,L");
    table[0x96] = Instruction::new(4, 1, Cpu::sub_a_r8, "SUB A,[HL]");
    table[0x97] = Instruction::new(4, 1, Cpu::sub_a_r8, "SUB A,A");

    // --- SUB A, n8 ---
    table[0xD6] = Instruction::new(8, 2, Cpu::sub_a_n8, "SUB A,n8");

    // --- SBC A, r8 ---
    table[0x98] = Instruction::new(4, 1, Cpu::sbc_a_r8, "SBC A,B");
    table[0x99] = Instruction::new(4, 1, Cpu::sbc_a_r8, "SBC A,C");
    table[0x9A] = Instruction::new(4, 1, Cpu::sbc_a_r8, "SBC A,D");
    table[0x9B] = Instruction::new(4, 1, Cpu::sbc_a_r8, "SBC A,E");
    table[0x9C] = Instruction::new(4, 1, Cpu::sbc_a_r8, "SBC A,H");
    table[0x9D] = Instruction::new(4, 1, Cpu::sbc_a_r8, "SBC A,L");
    table[0x9E] = Instruction::new(4, 1, Cpu::sbc_a_r8, "SBC A,[HL]");
    table[0x9F] = Instruction::new(4, 1, Cpu::sbc_a_r8, "SBC A,A");

    // --- SBC A, n8 ---
    table[0xDE] = Instruction::new(8, 2, Cpu::sbc_a_n8, "SBC A,n8");

    // --- AND A, r8 ---
    table[0xA0] = Instruction::new(4, 1, Cpu::and_a_r8, "AND A,B");
    table[0xA1] = Instruction::new(4, 1, Cpu::and_a_r8, "AND A,C");
    table[0xA2] = Instruction::new(4, 1, Cpu::and_a_r8, "AND A,D");
    table[0xA3] = Instruction::new(4, 1, Cpu::and_a_r8, "AND A,E");
    table[0xA4] = Instruction::new(4, 1, Cpu::and_a_r8, "AND A,H");
    table[0xA5] = Instruction::new(4, 1, Cpu::and_a_r8, "AND A,L");
    table[0xA6] = Instruction::new(4, 1, Cpu::and_a_r8, "AND A,[HL]");
    table[0xA7] = Instruction::new(4, 1, Cpu::and_a_r8, "AND A,A");

    // --- AND A, n8 ---
    table[0xE6] = Instruction::new(8, 2, Cpu::and_a_n8, "AND A,n8");

    // --- XOR A, r8 ---
    table[0xA8] = Instruction::new(4, 1, Cpu::xor_a_r8, "XOR A,B");
    table[0xA9] = Instruction::new(4, 1, Cpu::xor_a_r8, "XOR A,C");
    table[0xAA] = Instruction::new(4, 1, Cpu::xor_a_r8, "XOR A,D");
    table[0xAB] = Instruction::new(4, 1, Cpu::xor_a_r8, "XOR A,E");
    table[0xAC] = Instruction::new(4, 1, Cpu::xor_a_r8, "XOR A,H");
    table[0xAD] = Instruction::new(4, 1, Cpu::xor_a_r8, "XOR A,L");
    table[0xAE] = Instruction::new(4, 1, Cpu::xor_a_r8, "XOR A,[HL]");
    table[0xAF] = Instruction::new(4, 1, Cpu::xor_a_r8, "XOR A,A");

    // --- XOR A, n8 ---
    table[0xEE] = Instruction::new(8, 2, Cpu::xor_a_n8, "XOR A,n8");

    // --- OR A, r8 ---
    table[0xB0] = Instruction::new(4, 1, Cpu::or_a_r8, "OR A,B");
    table[0xB1] = Instruction::new(4, 1, Cpu::or_a_r8, "OR A,C");
    table[0xB2] = Instruction::new(4, 1, Cpu::or_a_r8, "OR A,D");
    table[0xB3] = Instruction::new(4, 1, Cpu::or_a_r8, "OR A,E");
    table[0xB4] = Instruction::new(4, 1, Cpu::or_a_r8, "OR A,H");
    table[0xB5] = Instruction::new(4, 1, Cpu::or_a_r8, "OR A,L");
    table[0xB6] = Instruction::new(4, 1, Cpu::or_a_r8, "OR A,[HL]");
    table[0xB7] = Instruction::new(4, 1, Cpu::or_a_r8, "OR A,A");

    // --- OR A, n8 ---
    table[0xF6] = Instruction::new(8, 2, Cpu::or_a_n8, "OR A,n8");

    // --- CPL ---
    table[0x2F] = Instruction::new(4, 1, Cpu::cpl, "CPL");

    // --- CP A, r8 ---
    table[0xB8] = Instruction::new(4, 1, Cpu::cp_a_r8, "CP A,B");
    table[0xB9] = Instruction::new(4, 1, Cpu::cp_a_r8, "CP A,C");
    table[0xBA] = Instruction::new(4, 1, Cpu::cp_a_r8, "CP A,D");
    table[0xBB] = Instruction::new(4, 1, Cpu::cp_a_r8, "CP A,E");
    table[0xBC] = Instruction::new(4, 1, Cpu::cp_a_r8, "CP A,H");
    table[0xBD] = Instruction::new(4, 1, Cpu::cp_a_r8, "CP A,L");
    table[0xBE] = Instruction::new(4, 1, Cpu::cp_a_r8, "CP A,[HL]");
    table[0xBF] = Instruction::new(4, 1, Cpu::cp_a_r8, "CP A,A");

    // --- CP A, n8 ---
    table[0xFE] = Instruction::new(8, 2, Cpu::cp_a_n8, "CP A,n8");

    // --- ADD HL, r16 ---
    table[0x09] = Instruction::new(8, 1, Cpu::add_hl_r16, "ADD HL,BC");
    table[0x19] = Instruction::new(8, 1, Cpu::add_hl_r16, "ADD HL,DE");
    table[0x29] = Instruction::new(8, 1, Cpu::add_hl_r16, "ADD HL,HL");
    table[0x39] = Instruction::new(8, 1, Cpu::add_hl_r16, "ADD HL,SP");

    // --- ADD SP, e8 ---
    table[0xE8] = Instruction::new(16, 2, Cpu::add_sp_e8, "ADD SP,e8");

    // --- INC r8 ---
    table[0x04] = Instruction::new(4, 1, Cpu::inc_r8, "INC B");
    table[0x0C] = Instruction::new(4, 1, Cpu::inc_r8, "INC C");
    table[0x14] = Instruction::new(4, 1, Cpu::inc_r8, "INC D");
    table[0x1C] = Instruction::new(4, 1, Cpu::inc_r8, "INC E");
    table[0x24] = Instruction::new(4, 1, Cpu::inc_r8, "INC H");
    table[0x2C] = Instruction::new(4, 1, Cpu::inc_r8, "INC L");
    table[0x34] = Instruction::new(4, 1, Cpu::inc_r8, "INC [HL]");
    table[0x3C] = Instruction::new(4, 1, Cpu::inc_r8, "INC A");

    // --- INC r16 ---
    table[0x03] = Instruction::new(8, 1, Cpu::inc_r16, "INC BC");
    table[0x13] = Instruction::new(8, 1, Cpu::inc_r16, "INC DE");
    table[0x23] = Instruction::new(8, 1, Cpu::inc_r16, "INC HL");
    table[0x33] = Instruction::new(8, 1, Cpu::inc_r16, "INC SP");

    // --- DEC r8 ---
    table[0x05] = Instruction::new(4, 1, Cpu::dec_r8, "DEC B");
    table[0x0D] = Instruction::new(4, 1, Cpu::dec_r8, "DEC C");
    table[0x15] = Instruction::new(4, 1, Cpu::dec_r8, "DEC D");
    table[0x1D] = Instruction::new(4, 1, Cpu::dec_r8, "DEC E");
    table[0x25] = Instruction::new(4, 1, Cpu::dec_r8, "DEC H");
    table[0x2D] = Instruction::new(4, 1, Cpu::dec_r8, "DEC L");
    table[0x35] = Instruction::new(4, 1, Cpu::dec_r8, "DEC [HL]");
    table[0x3D] = Instruction::new(4, 1, Cpu::dec_r8, "DEC A");

    // --- DEC r16 ---
    table[0x0B] = Instruction::new(8, 1, Cpu::dec_r16, "DEC BC");
    table[0x1B] = Instruction::new(8, 1, Cpu::dec_r16, "DEC DE");
    table[0x2B] = Instruction::new(8, 1, Cpu::dec_r16, "DEC HL");
    table[0x3B] = Instruction::new(8, 1, Cpu::dec_r16, "DEC SP");

    // --- RLA ---
    table[0x17] = Instruction::new(4, 1, Cpu::rla, "RLA");

    // --- RRA ---
    table[0x1F] = Instruction::new(4, 1, Cpu::rra, "RRA");

    // --- RLCA ---
    table[0x07] = Instruction::new(4, 1, Cpu::rlca, "RLCA");

    // --- RRCA ---
    table[0x0F] = Instruction::new(4, 1, Cpu::rrca, "RRCA");

    // --- DAA ---
    table[0x27] = Instruction::new(4, 1, Cpu::daa, "DAA");

    // --- Carry flag ---
    table[0x37] = Instruction::new(4, 1, Cpu::scf, "SCF");
    table[0x3F] = Instruction::new(4, 1, Cpu::ccf, "CCF");

    table
}
