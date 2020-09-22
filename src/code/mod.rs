#[cfg(test)]
mod test;
pub enum Instruction {
    Constant(u16),
}
impl Instruction {
    fn opcode_byte(&self) -> u8 {
        match self {
            Self::Constant(_) => 0,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut operand_bytes = match self {
            Self::Constant(constant) => constant.to_be_bytes().to_vec(),
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
            0 => {
                // Constant. Read 2 bytes.
                let first = *iter.next().unwrap();
                let second = *iter.next().unwrap();
                let constant = u16::from_be_bytes([first, second]);
                Some(Self::Constant(constant))
            }
            _ => panic!("Unknown op byte"),
        }
    }
}
