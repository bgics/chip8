#[derive(Debug)]
pub enum Instruction {
    Cls,
    Ret,
    Jp { addr: u16 },
    JpV0 { addr: u16 },
    LdI { addr: u16 },
    LdByte { vx: u8, byte: u8 },
    LdReg { vx: u8, vy: u8 },
    LdRegDt { vx: u8 },
    LdDt { vx: u8 },
    LdSt { vx: u8 },
    LdFont { vx: u8 },
    Rnd { vx: u8, byte: u8 },
    AddByte { vx: u8, byte: u8 },
    AddI { vx: u8 },
    AndReg { vx: u8, vy: u8 },
    XorReg { vx: u8, vy: u8 },
    AddRegCarry { vx: u8, vy: u8 },
    SubReg { vx: u8, vy: u8 },
    SubNReg { vx: u8, vy: u8 },
    Shr { vx: u8 },
    Shl { vx: u8 },
    OrReg { vx: u8, vy: u8 },
    SeByte { vx: u8, byte: u8 },
    SeReg { vx: u8, vy: u8 },
    SneByte { vx: u8, byte: u8 },
    SneReg { vx: u8, vy: u8 },
    Skp { vx: u8 },
    Sknp { vx: u8 },
    KeyWait { vx: u8 },
    Store { vx: u8 },
    StoreBcd { vx: u8 },
    Read { vx: u8 },
    Call { addr: u16 },
    Drw { vx: u8, vy: u8, nibble: u8 },
    Unknown { instruction: u16 },
}

impl From<u16> for Instruction {
    fn from(opcode: u16) -> Self {
        if opcode == 0x00E0 {
            Instruction::Cls
        } else if opcode == 0x00EE {
            Instruction::Ret
        } else if opcode >> 12 == 0x1 {
            let addr = opcode & 0xFFF;

            Instruction::Jp { addr }
        } else if opcode >> 12 == 0xB {
            let addr = opcode & 0xFFF;

            Instruction::JpV0 { addr }
        } else if opcode >> 12 == 0x2 {
            let addr = opcode & 0xFFF;

            Instruction::Call { addr }
        } else if opcode >> 12 == 0x6 {
            let vx = ((opcode >> 8) & 0x0F) as u8;
            let byte = (opcode & 0xFF) as u8;

            Instruction::LdByte { vx, byte }
        } else if opcode >> 12 == 0x7 {
            let vx = ((opcode >> 8) & 0x0F) as u8;
            let byte = (opcode & 0xFF) as u8;

            Instruction::AddByte { vx, byte }
        } else if opcode >> 12 == 0xF && opcode & 0xFF == 0x1E {
            let vx = ((opcode >> 8) & 0x0F) as u8;

            Instruction::AddI { vx }
        } else if opcode >> 12 == 0xA {
            let addr = opcode & 0xFFF;

            Instruction::LdI { addr }
        } else if opcode >> 12 == 0xD {
            let vx = ((opcode >> 8) & 0x0F) as u8;
            let vy = ((opcode >> 4) & 0x0F) as u8;
            let nibble = (opcode & 0x0F) as u8;

            Instruction::Drw { vx, vy, nibble }
        } else if opcode >> 12 == 0x3 {
            let vx = ((opcode >> 8) & 0x0F) as u8;
            let byte = (opcode & 0xFF) as u8;

            Instruction::SeByte { vx, byte }
        } else if opcode >> 12 == 0x4 {
            let vx = ((opcode >> 8) & 0x0F) as u8;
            let byte = (opcode & 0xFF) as u8;

            Instruction::SneByte { vx, byte }
        } else if opcode >> 12 == 0xC {
            let vx = ((opcode >> 8) & 0x0F) as u8;
            let byte = (opcode & 0xFF) as u8;

            Instruction::Rnd { vx, byte }
        } else if opcode >> 12 == 0x5 && opcode & 0x0F == 0x0 {
            let vx = ((opcode >> 8) & 0x0F) as u8;
            let vy = ((opcode >> 4) & 0x0F) as u8;

            Instruction::SeReg { vx, vy }
        } else if opcode >> 12 == 0x9 && opcode & 0x0F == 0x0 {
            let vx = ((opcode >> 8) & 0x0F) as u8;
            let vy = ((opcode >> 4) & 0x0F) as u8;

            Instruction::SneReg { vx, vy }
        } else if opcode >> 12 == 0x8 && opcode & 0x0F == 0x0 {
            let vx = ((opcode >> 8) & 0x0F) as u8;
            let vy = ((opcode >> 4) & 0x0F) as u8;

            Instruction::LdReg { vx, vy }
        } else if opcode >> 12 == 0x8 && opcode & 0x0F == 0x1 {
            let vx = ((opcode >> 8) & 0x0F) as u8;
            let vy = ((opcode >> 4) & 0x0F) as u8;

            Instruction::OrReg { vx, vy }
        } else if opcode >> 12 == 0x8 && opcode & 0x0F == 0x2 {
            let vx = ((opcode >> 8) & 0x0F) as u8;
            let vy = ((opcode >> 4) & 0x0F) as u8;

            Instruction::AndReg { vx, vy }
        } else if opcode >> 12 == 0x8 && opcode & 0x0F == 0x3 {
            let vx = ((opcode >> 8) & 0x0F) as u8;
            let vy = ((opcode >> 4) & 0x0F) as u8;

            Instruction::XorReg { vx, vy }
        } else if opcode >> 12 == 0x8 && opcode & 0x0F == 0x4 {
            let vx = ((opcode >> 8) & 0x0F) as u8;
            let vy = ((opcode >> 4) & 0x0F) as u8;

            Instruction::AddRegCarry { vx, vy }
        } else if opcode >> 12 == 0x8 && opcode & 0x0F == 0x5 {
            let vx = ((opcode >> 8) & 0x0F) as u8;
            let vy = ((opcode >> 4) & 0x0F) as u8;

            Instruction::SubReg { vx, vy }
        } else if opcode >> 12 == 0x8 && opcode & 0x0F == 0x6 {
            let vx = ((opcode >> 8) & 0x0F) as u8;

            Instruction::Shr { vx }
        } else if opcode >> 12 == 0x8 && opcode & 0x0F == 0x7 {
            let vx = ((opcode >> 8) & 0x0F) as u8;
            let vy = ((opcode >> 4) & 0x0F) as u8;

            Instruction::SubNReg { vx, vy }
        } else if opcode >> 12 == 0x8 && opcode & 0x0F == 0xE {
            let vx = ((opcode >> 8) & 0x0F) as u8;

            Instruction::Shl { vx }
        } else if opcode >> 12 == 0xE && opcode & 0xFF == 0x9E {
            let vx = ((opcode >> 8) & 0x0F) as u8;

            Instruction::Skp { vx }
        } else if opcode >> 12 == 0xE && opcode & 0xFF == 0xA1 {
            let vx = ((opcode >> 8) & 0x0F) as u8;

            Instruction::Sknp { vx }
        } else if opcode >> 12 == 0xF && opcode & 0xFF == 0x0A {
            let vx = ((opcode >> 8) & 0x0F) as u8;

            Instruction::KeyWait { vx }
        } else if opcode >> 12 == 0xF && opcode & 0xFF == 0x55 {
            let vx = ((opcode >> 8) & 0x0F) as u8;

            Instruction::Store { vx }
        } else if opcode >> 12 == 0xF && opcode & 0xFF == 0x65 {
            let vx = ((opcode >> 8) & 0x0F) as u8;

            Instruction::Read { vx }
        } else if opcode >> 12 == 0xF && opcode & 0xFF == 0x33 {
            let vx = ((opcode >> 8) & 0x0F) as u8;

            Instruction::StoreBcd { vx }
        } else if opcode >> 12 == 0xF && opcode & 0xFF == 0x07 {
            let vx = ((opcode >> 8) & 0x0F) as u8;

            Instruction::LdRegDt { vx }
        } else if opcode >> 12 == 0xF && opcode & 0xFF == 0x29 {
            let vx = ((opcode >> 8) & 0x0F) as u8;

            Instruction::LdFont { vx }
        } else if opcode >> 12 == 0xF && opcode & 0xFF == 0x15 {
            let vx = ((opcode >> 8) & 0x0F) as u8;

            Instruction::LdDt { vx }
        } else if opcode >> 12 == 0xF && opcode & 0xFF == 0x18 {
            let vx = ((opcode >> 8) & 0x0F) as u8;

            Instruction::LdSt { vx }
        } else {
            Instruction::Unknown {
                instruction: opcode,
            }
        }
    }
}
