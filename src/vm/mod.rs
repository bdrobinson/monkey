use crate::{code, compiler, logic, object};
use object::Object;
use std::rc::Rc;

const STACK_SIZE: usize = 2048;

struct Stack<'a> {
    elements: Vec<Rc<Object<'a>>>,
}

impl<'a> Stack<'a> {
    fn new() -> Self {
        Stack {
            elements: Vec::with_capacity(STACK_SIZE),
        }
    }
    fn push(&mut self, obj: Rc<Object<'a>>) {
        self.elements.push(obj);
    }
    fn pop(&mut self) -> Option<Rc<Object<'a>>> {
        let el = self.elements.pop();
        el
    }
}

#[derive(Debug)]
pub enum VmError {
    PopEmptyStack,
    Misc(String),
}

pub struct Vm<'ast, 'bytecode>
where
    'ast: 'bytecode,
{
    bytecode: &'bytecode compiler::Bytecode<'ast>,
    stack: Stack<'ast>,
}

impl<'ast, 'bytecode> Vm<'ast, 'bytecode> {
    pub fn new(bytecode: &'bytecode compiler::Bytecode<'ast>) -> Vm<'ast, 'bytecode> {
        Vm {
            bytecode,
            stack: Stack::new(),
        }
    }

    pub fn run(&mut self) -> Result<Option<Rc<Object<'ast>>>, VmError> {
        let mut instructions_iter = self.bytecode.instructions.iter();
        let mut should_continue = true;
        let mut last_popped: Option<Rc<Object>> = None;
        while should_continue {
            let instruction = code::Instruction::from_bytes(&mut instructions_iter);
            if let Some(instruction) = instruction {
                match instruction {
                    code::Instruction::Constant(constant_index) => {
                        self.stack
                            .push(Rc::clone(&self.bytecode.constants[constant_index as usize]));
                    }
                    code::Instruction::Add => {
                        self.handle_infix(&logic::InfixOperator::Plus)?;
                    }
                    code::Instruction::Sub => {
                        self.handle_infix(&logic::InfixOperator::Minus)?;
                    }
                    code::Instruction::Mul => {
                        self.handle_infix(&logic::InfixOperator::Multiply)?;
                    }
                    code::Instruction::Div => {
                        self.handle_infix(&logic::InfixOperator::Divide)?;
                    }
                    code::Instruction::Pop => {
                        last_popped = self.stack.pop();
                    }
                    code::Instruction::True => {
                        self.stack.push(Rc::new(Object::Boolean(true)));
                    }
                    code::Instruction::False => {
                        self.stack.push(Rc::new(Object::Boolean(false)));
                    }
                    code::Instruction::Equal => {
                        self.handle_infix(&logic::InfixOperator::Eq)?;
                    }
                    code::Instruction::NotEqual => {
                        self.handle_infix(&logic::InfixOperator::NotEq)?;
                    }
                    code::Instruction::GreaterThan => {
                        self.handle_infix(&logic::InfixOperator::Gt)?;
                    }
                    code::Instruction::Minus => {
                        self.handle_prefix(&logic::PrefixOperator::Minus)?;
                    }
                    code::Instruction::Bang => {
                        self.handle_prefix(&logic::PrefixOperator::Bang)?;
                    }
                    code::Instruction::JumpFalse(_) => todo!(),
                    code::Instruction::Jump(_) => todo!(),
                }
            } else {
                should_continue = false;
            }
        }
        Ok(last_popped)
    }
    fn handle_prefix(&mut self, operator: &logic::PrefixOperator) -> Result<(), VmError> {
        let operand = self.try_pop()?;
        let result = logic::eval_prefix(operand, operator).map_err(VmError::Misc)?;
        self.stack.push(Rc::new(result));
        Ok(())
    }
    fn handle_infix(&mut self, operator: &logic::InfixOperator) -> Result<(), VmError> {
        let right = self.try_pop()?;
        let left = self.try_pop()?;
        let result = logic::eval_infix(left, operator, right).map_err(VmError::Misc)?;
        self.stack.push(Rc::new(result));
        Ok(())
    }
    fn try_pop(&mut self) -> Result<Rc<Object<'ast>>, VmError> {
        self.stack.pop().ok_or(VmError::PopEmptyStack)
    }
}

#[cfg(test)]
mod test {
    use crate::{compiler, lexer, object, parser, vm};
    use object::Object;
    struct VmTestCase<'a> {
        input: &'static str,
        expected: Object<'a>,
    }

    fn run_vm_test(case: VmTestCase) {
        let mut lexer = lexer::new(case.input);
        let mut parser = parser::Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        let bytecode = compiler::compile_program(&program);
        let mut vm = vm::Vm::new(&bytecode);
        let object = vm.run().unwrap();
        assert_eq!(object.as_deref(), Some(&case.expected));
    }

    #[test]
    fn vm_tests() {
        let tests = vec![
            VmTestCase {
                input: "3 + 4",
                expected: Object::Integer(7),
            },
            VmTestCase {
                input: "5 - 3",
                expected: Object::Integer(2),
            },
            VmTestCase {
                input: "5 * 3",
                expected: Object::Integer(15),
            },
            VmTestCase {
                input: "20 / 2",
                expected: Object::Integer(10),
            },
            VmTestCase {
                input: "true",
                expected: Object::Boolean(true),
            },
            VmTestCase {
                input: "false",
                expected: Object::Boolean(false),
            },
            // Comparison ops
            VmTestCase {
                input: "1==1",
                expected: Object::Boolean(true),
            },
            VmTestCase {
                input: "1==2",
                expected: Object::Boolean(false),
            },
            VmTestCase {
                input: "1!=2",
                expected: Object::Boolean(true),
            },
            VmTestCase {
                input: "1!=1",
                expected: Object::Boolean(false),
            },
            VmTestCase {
                input: "2>1",
                expected: Object::Boolean(true),
            },
            VmTestCase {
                input: "2>5",
                expected: Object::Boolean(false),
            },
            VmTestCase {
                input: "2<5",
                expected: Object::Boolean(true),
            },
            VmTestCase {
                input: "2<1",
                expected: Object::Boolean(false),
            },
            // Prefix operators
            VmTestCase {
                input: "!true",
                expected: Object::Boolean(false),
            },
            VmTestCase {
                input: "!false",
                expected: Object::Boolean(true),
            },
            VmTestCase {
                input: "-3",
                expected: Object::Integer(-3),
            },
        ];
        for test in tests {
            run_vm_test(test);
        }
    }
}
