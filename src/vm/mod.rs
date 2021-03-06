use crate::{code, compiler, logic, object};
use std::rc::Rc;

const STACK_SIZE: usize = 2048;

struct Stack<'a> {
    elements: Vec<Rc<object::Object<'a>>>,
    last_popped: Option<Rc<object::Object<'a>>>,
}

impl<'a> Stack<'a> {
    fn new() -> Self {
        Stack {
            elements: Vec::with_capacity(STACK_SIZE),
            last_popped: None,
        }
    }
    fn push(&mut self, obj: Rc<object::Object<'a>>) {
        self.elements.push(obj);
    }
    fn pop(&mut self) -> Option<Rc<object::Object<'a>>> {
        let el = self.elements.pop();
        self.last_popped = el.clone();
        el
    }
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

    pub fn run(&mut self) -> Result<(), String> {
        let mut instructions_iter = self.bytecode.instructions.iter();
        let mut should_continue = true;
        while should_continue {
            let instruction = code::Instruction::from_bytes(&mut instructions_iter);
            if let Some(instruction) = instruction {
                match instruction {
                    code::Instruction::Constant(constant_index) => {
                        self.stack
                            .push(Rc::clone(&self.bytecode.constants[constant_index as usize]));
                    }
                    code::Instruction::Add => {
                        let right = self.stack.pop().unwrap();
                        let left = self.stack.pop().unwrap();
                        let result = logic::eval_infix(left, &logic::InfixOperator::Plus, right)?;
                        self.stack.push(Rc::new(result));
                    }
                    code::Instruction::Pop => {
                        self.stack.pop();
                    }
                }
            } else {
                should_continue = false;
            }
        }
        Ok(())
    }

    #[cfg(test)]
    pub fn last_popped(&self) -> Option<Rc<object::Object<'ast>>> {
        self.stack.last_popped.clone()
    }
}

#[cfg(test)]
mod test {
    use crate::{compiler, lexer, object, parser, vm};
    struct VmTestCase<'a> {
        input: &'static str,
        expected: object::Object<'a>,
    }

    fn run_vm_test(case: VmTestCase) {
        let mut lexer = lexer::new(case.input);
        let mut parser = parser::Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        let bytecode = compiler::compile_program(&program);
        let mut vm = vm::Vm::new(&bytecode);
        vm.run().unwrap();
        let last_popped = vm.last_popped();
        assert_eq!(last_popped.as_deref(), Some(&case.expected));
    }

    #[test]
    fn vm_tests() {
        let tests = vec![VmTestCase {
            input: "3 + 4",
            expected: object::Object::Integer(7),
        }];
        for test in tests {
            run_vm_test(test);
        }
    }
}
