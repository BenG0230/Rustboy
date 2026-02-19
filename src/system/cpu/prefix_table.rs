use crate::system::cpu::{Cpu, decode::Instruction};

pub const fn load_prefix_table() -> [Instruction; 256] {
    let unknown_instruction = Instruction::new(
        0,
        1,
        Cpu::unknown_pre_instr,
        "Unknown Prefixed Instruction!",
    );
    let mut table = [unknown_instruction; 256];

    // --- RLC r8 ---
    table[0x00] = Instruction::new(8, 1, Cpu::rlc_r8, "RLC B");
    table[0x01] = Instruction::new(8, 1, Cpu::rlc_r8, "RLC C");
    table[0x02] = Instruction::new(8, 1, Cpu::rlc_r8, "RLC D");
    table[0x03] = Instruction::new(8, 1, Cpu::rlc_r8, "RLC E");
    table[0x04] = Instruction::new(8, 1, Cpu::rlc_r8, "RLC H");
    table[0x05] = Instruction::new(8, 1, Cpu::rlc_r8, "RLC L");
    table[0x06] = Instruction::new(8, 1, Cpu::rlc_r8, "RLC [HL]");
    table[0x07] = Instruction::new(8, 1, Cpu::rlc_r8, "RLC A");

    // --- RRC r8 ---
    table[0x08] = Instruction::new(8, 1, Cpu::rrc_r8, "RRC B");
    table[0x09] = Instruction::new(8, 1, Cpu::rrc_r8, "RRC C");
    table[0x0A] = Instruction::new(8, 1, Cpu::rrc_r8, "RRC D");
    table[0x0B] = Instruction::new(8, 1, Cpu::rrc_r8, "RRC E");
    table[0x0C] = Instruction::new(8, 1, Cpu::rrc_r8, "RRC H");
    table[0x0D] = Instruction::new(8, 1, Cpu::rrc_r8, "RRC L");
    table[0x0E] = Instruction::new(8, 1, Cpu::rrc_r8, "RRC [HL]");
    table[0x0F] = Instruction::new(8, 1, Cpu::rrc_r8, "RRC A");

    // --- RL r8 ---
    table[0x10] = Instruction::new(8, 1, Cpu::rl_r8, "RL B");
    table[0x11] = Instruction::new(8, 1, Cpu::rl_r8, "RL C");
    table[0x12] = Instruction::new(8, 1, Cpu::rl_r8, "RL D");
    table[0x13] = Instruction::new(8, 1, Cpu::rl_r8, "RL E");
    table[0x14] = Instruction::new(8, 1, Cpu::rl_r8, "RL H");
    table[0x15] = Instruction::new(8, 1, Cpu::rl_r8, "RL L");
    table[0x16] = Instruction::new(8, 1, Cpu::rl_r8, "RL [HL]");
    table[0x17] = Instruction::new(8, 1, Cpu::rl_r8, "RL A");

    // --- RR r8 ---
    table[0x18] = Instruction::new(8, 1, Cpu::rr_r8, "RR B");
    table[0x19] = Instruction::new(8, 1, Cpu::rr_r8, "RR C");
    table[0x1A] = Instruction::new(8, 1, Cpu::rr_r8, "RR D");
    table[0x1B] = Instruction::new(8, 1, Cpu::rr_r8, "RR E");
    table[0x1C] = Instruction::new(8, 1, Cpu::rr_r8, "RR H");
    table[0x1D] = Instruction::new(8, 1, Cpu::rr_r8, "RR L");
    table[0x1E] = Instruction::new(8, 1, Cpu::rr_r8, "RR [HL]");
    table[0x1F] = Instruction::new(8, 1, Cpu::rr_r8, "RR A");

    // --- SLA r8 ---
    table[0x20] = Instruction::new(8, 1, Cpu::sla_r8, "SLA B");
    table[0x21] = Instruction::new(8, 1, Cpu::sla_r8, "SLA C");
    table[0x22] = Instruction::new(8, 1, Cpu::sla_r8, "SLA D");
    table[0x23] = Instruction::new(8, 1, Cpu::sla_r8, "SLA E");
    table[0x24] = Instruction::new(8, 1, Cpu::sla_r8, "SLA H");
    table[0x25] = Instruction::new(8, 1, Cpu::sla_r8, "SLA L");
    table[0x26] = Instruction::new(8, 1, Cpu::sla_r8, "SLA [HL]");
    table[0x27] = Instruction::new(8, 1, Cpu::sla_r8, "SLA A");

    // --- SRA r8 ---
    table[0x28] = Instruction::new(8, 1, Cpu::sra_r8, "SRA B");
    table[0x29] = Instruction::new(8, 1, Cpu::sra_r8, "SRA C");
    table[0x2A] = Instruction::new(8, 1, Cpu::sra_r8, "SRA D");
    table[0x2B] = Instruction::new(8, 1, Cpu::sra_r8, "SRA E");
    table[0x2C] = Instruction::new(8, 1, Cpu::sra_r8, "SRA H");
    table[0x2D] = Instruction::new(8, 1, Cpu::sra_r8, "SRA L");
    table[0x2E] = Instruction::new(8, 1, Cpu::sra_r8, "SRA [HL]");
    table[0x2F] = Instruction::new(8, 1, Cpu::sra_r8, "SRA A");

    // --- SWAP r8 ---
    table[0x30] = Instruction::new(8, 1, Cpu::swap_r8, "SWAP B");
    table[0x31] = Instruction::new(8, 1, Cpu::swap_r8, "SWAP C");
    table[0x32] = Instruction::new(8, 1, Cpu::swap_r8, "SWAP D");
    table[0x33] = Instruction::new(8, 1, Cpu::swap_r8, "SWAP E");
    table[0x34] = Instruction::new(8, 1, Cpu::swap_r8, "SWAP H");
    table[0x35] = Instruction::new(8, 1, Cpu::swap_r8, "SWAP L");
    table[0x36] = Instruction::new(8, 1, Cpu::swap_r8, "SWAP [HL]");
    table[0x37] = Instruction::new(8, 1, Cpu::swap_r8, "SWAP A");

    // --- SRL r8 ---
    table[0x38] = Instruction::new(8, 1, Cpu::srl_r8, "SRL B");
    table[0x39] = Instruction::new(8, 1, Cpu::srl_r8, "SRL C");
    table[0x3A] = Instruction::new(8, 1, Cpu::srl_r8, "SRL D");
    table[0x3B] = Instruction::new(8, 1, Cpu::srl_r8, "SRL E");
    table[0x3C] = Instruction::new(8, 1, Cpu::srl_r8, "SRL H");
    table[0x3D] = Instruction::new(8, 1, Cpu::srl_r8, "SRL L");
    table[0x3E] = Instruction::new(8, 1, Cpu::srl_r8, "SRL [HL]");
    table[0x3F] = Instruction::new(8, 1, Cpu::srl_r8, "SRL A");

    // --- BIT b3,r8 ---
    table[0x40] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 0,B");
    table[0x41] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 0,C");
    table[0x42] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 0,D");
    table[0x43] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 0,E");
    table[0x44] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 0,H");
    table[0x45] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 0,L");
    table[0x46] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 0,[HL]");
    table[0x47] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 0,A");

    table[0x48] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 1,B");
    table[0x49] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 1,C");
    table[0x4A] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 1,D");
    table[0x4B] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 1,E");
    table[0x4C] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 1,H");
    table[0x4D] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 1,L");
    table[0x4E] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 1,[HL]");
    table[0x4F] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 1,A");

    table[0x50] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 2,B");
    table[0x51] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 2,C");
    table[0x52] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 2,D");
    table[0x53] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 2,E");
    table[0x54] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 2,H");
    table[0x55] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 2,L");
    table[0x56] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 2,[HL]");
    table[0x57] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 2,A");

    table[0x58] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 3,B");
    table[0x59] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 3,C");
    table[0x5A] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 3,D");
    table[0x5B] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 3,E");
    table[0x5C] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 3,H");
    table[0x5D] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 3,L");
    table[0x5E] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 3,[HL]");
    table[0x5F] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 3,A");

    table[0x60] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 4,B");
    table[0x61] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 4,C");
    table[0x62] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 4,D");
    table[0x63] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 4,E");
    table[0x64] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 4,H");
    table[0x65] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 4,L");
    table[0x66] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 4,[HL]");
    table[0x67] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 4,A");

    table[0x68] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 5,B");
    table[0x69] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 5,C");
    table[0x6A] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 5,D");
    table[0x6B] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 5,E");
    table[0x6C] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 5,H");
    table[0x6D] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 5,L");
    table[0x6E] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 5,[HL]");
    table[0x6F] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 5,A");

    table[0x70] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 6,B");
    table[0x71] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 6,C");
    table[0x72] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 6,D");
    table[0x73] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 6,E");
    table[0x74] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 6,H");
    table[0x75] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 6,L");
    table[0x76] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 6,[HL]");
    table[0x77] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 6,A");

    table[0x78] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 7,B");
    table[0x79] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 7,C");
    table[0x7A] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 7,D");
    table[0x7B] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 7,E");
    table[0x7C] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 7,H");
    table[0x7D] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 7,L");
    table[0x7E] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 7,[HL]");
    table[0x7F] = Instruction::new(8, 1, Cpu::bit_b3_r8, "BIT 7,A");

    // --- RES b3,r3 ---
    table[0x80] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 0,B");
    table[0x81] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 0,C");
    table[0x82] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 0,D");
    table[0x83] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 0,E");
    table[0x84] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 0,H");
    table[0x85] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 0,L");
    table[0x86] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 0,[HL]");
    table[0x87] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 0,A");

    table[0x88] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 1,B");
    table[0x89] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 1,C");
    table[0x8A] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 1,D");
    table[0x8B] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 1,E");
    table[0x8C] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 1,H");
    table[0x8D] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 1,L");
    table[0x8E] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 1,[HL]");
    table[0x8F] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 1,A");

    table[0x90] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 2,B");
    table[0x91] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 2,C");
    table[0x92] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 2,D");
    table[0x93] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 2,E");
    table[0x94] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 2,H");
    table[0x95] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 2,L");
    table[0x96] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 2,[HL]");
    table[0x97] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 2,A");

    table[0x98] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 3,B");
    table[0x99] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 3,C");
    table[0x9A] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 3,D");
    table[0x9B] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 3,E");
    table[0x9C] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 3,H");
    table[0x9D] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 3,L");
    table[0x9E] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 3,[HL]");
    table[0x9F] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 3,A");

    table[0xA0] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 4,B");
    table[0xA1] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 4,C");
    table[0xA2] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 4,D");
    table[0xA3] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 4,E");
    table[0xA4] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 4,H");
    table[0xA5] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 4,L");
    table[0xA6] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 4,[HL]");
    table[0xA7] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 4,A");

    table[0xA8] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 5,B");
    table[0xA9] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 5,C");
    table[0xAA] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 5,D");
    table[0xAB] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 5,E");
    table[0xAC] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 5,H");
    table[0xAD] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 5,L");
    table[0xAE] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 5,[HL]");
    table[0xAF] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 5,A");

    table[0xB0] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 6,B");
    table[0xB1] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 6,C");
    table[0xB2] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 6,D");
    table[0xB3] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 6,E");
    table[0xB4] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 6,H");
    table[0xB5] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 6,L");
    table[0xB6] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 6,[HL]");
    table[0xB7] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 6,A");

    table[0xB8] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 7,B");
    table[0xB9] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 7,C");
    table[0xBA] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 7,D");
    table[0xBB] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 7,E");
    table[0xBC] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 7,H");
    table[0xBD] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 7,L");
    table[0xBE] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 7,[HL]");
    table[0xBF] = Instruction::new(8, 1, Cpu::res_b3_r8, "RES 7,A");

    // --- SET b3,r8 ---
    table[0xC0] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 0,B");
    table[0xC1] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 0,C");
    table[0xC2] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 0,D");
    table[0xC3] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 0,E");
    table[0xC4] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 0,H");
    table[0xC5] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 0,L");
    table[0xC6] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 0,[HL]");
    table[0xC7] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 0,A");

    table[0xC8] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 1,B");
    table[0xC9] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 1,C");
    table[0xCA] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 1,D");
    table[0xCB] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 1,E");
    table[0xCC] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 1,H");
    table[0xCD] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 1,L");
    table[0xCE] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 1,[HL]");
    table[0xCF] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 1,A");

    table[0xD0] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 2,B");
    table[0xD1] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 2,C");
    table[0xD2] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 2,D");
    table[0xD3] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 2,E");
    table[0xD4] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 2,H");
    table[0xD5] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 2,L");
    table[0xD6] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 2,[HL]");
    table[0xD7] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 2,A");

    table[0xD8] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 3,B");
    table[0xD9] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 3,C");
    table[0xDA] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 3,D");
    table[0xDB] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 3,E");
    table[0xDC] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 3,H");
    table[0xDD] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 3,L");
    table[0xDE] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 3,[HL]");
    table[0xDF] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 3,A");

    table[0xE0] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 4,B");
    table[0xE1] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 4,C");
    table[0xE2] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 4,D");
    table[0xE3] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 4,E");
    table[0xE4] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 4,H");
    table[0xE5] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 4,L");
    table[0xE6] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 4,[HL]");
    table[0xE7] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 4,A");

    table[0xE8] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 5,B");
    table[0xE9] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 5,C");
    table[0xEA] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 5,D");
    table[0xEB] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 5,E");
    table[0xEC] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 5,H");
    table[0xED] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 5,L");
    table[0xEE] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 5,[HL]");
    table[0xEF] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 5,A");

    table[0xF0] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 6,B");
    table[0xF1] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 6,C");
    table[0xF2] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 6,D");
    table[0xF3] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 6,E");
    table[0xF4] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 6,H");
    table[0xF5] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 6,L");
    table[0xF6] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 6,[HL]");
    table[0xF7] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 6,A");

    table[0xF8] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 7,B");
    table[0xF9] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 7,C");
    table[0xFA] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 7,D");
    table[0xFB] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 7,E");
    table[0xFC] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 7,H");
    table[0xFD] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 7,L");
    table[0xFE] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 7,[HL]");
    table[0xFF] = Instruction::new(8, 1, Cpu::set_b3_r8, "SET 7,A");

    table
}
