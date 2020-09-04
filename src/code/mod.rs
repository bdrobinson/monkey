mod test;

pub enum OpCodeAndOperands {
    Constant(u16),
}
impl OpCodeAndOperands {
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
}
