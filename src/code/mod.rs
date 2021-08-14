#[cfg(test)]
mod test;
pub enum Instruction {
    Constant(u16),
    Add,
    Sub,
    Pop,
    Mul,
    Div,
    True,
    False,
    Equal,
    NotEqual,
    GreaterThan,
    Minus,
    Bang,
    JumpFalse(u16),
    Jump(u16),
}
impl Instruction {
    fn opcode_byte(&self) -> u8 {
        match self {
            Self::Constant(_) => 0,
            Self::Add => 1,
            Self::Sub => 2,
            Self::Pop => 3,
            Self::Mul => 4,
            Self::Div => 5,
            Self::True => 6,
            Self::False => 7,
            Self::Equal => 8,
            Self::NotEqual => 9,
            Self::GreaterThan => 10,
            Self::Minus => 11,
            Self::Bang => 12,
            Self::JumpFalse(_) => 13,
            Self::Jump(_) => 14,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut operand_bytes = match self {
            Self::Constant(constant) => constant.to_be_bytes().to_vec(),
            Self::Add => vec![],
            Self::Pop => vec![],
            Self::Sub => vec![],
            Self::Mul => vec![],
            Self::Div => vec![],
            Self::True => vec![],
            Self::False => vec![],
            Self::Equal => vec![],
            Self::NotEqual => vec![],
            Self::GreaterThan => vec![],
            Self::Minus => vec![],
            Self::Bang => vec![],
            Self::JumpFalse(position) => position.to_be_bytes().to_vec(),
            Self::Jump(position) => position.to_be_bytes().to_vec(),
        };
        let mut result = vec![self.opcode_byte()];
        Vec::append(&mut result, &mut operand_bytes);
        result
    }

    /**
     * Takes an iterator, reads some bytes and returns the corresponding instruction.
     */
    pub fn from_bytes(iter: &mut std::slice::Iter<u8>) -> Option<Self> {
        // If there is no next, return none to signify we've reached the end.
        let op_byte = iter.next()?;
        match op_byte {
            0 => Some(Self::Constant(read_2_bytes(iter))),
            1 => Some(Self::Add),
            2 => Some(Self::Sub),
            3 => Some(Self::Pop),
            4 => Some(Self::Mul),
            5 => Some(Self::Div),
            6 => Some(Self::True),
            7 => Some(Self::False),
            8 => Some(Self::Equal),
            9 => Some(Self::NotEqual),
            10 => Some(Self::GreaterThan),
            11 => Some(Self::Minus),
            12 => Some(Self::Bang),
            13 => Some(Self::JumpFalse(read_2_bytes(iter))),
            14 => Some(Self::Jump(read_2_bytes(iter))),
            _ => panic!("Unknown op byte"),
        }
    }
}

fn read_2_bytes(iter: &mut std::slice::Iter<u8>) -> u16 {
    let first = *iter.next().unwrap();
    let second = *iter.next().unwrap();
    u16::from_be_bytes([first, second])
}
