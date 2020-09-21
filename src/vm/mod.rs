use crate::{code, compiler, object};
use std::rc::Rc;

const STACK_SIZE: usize = 2048;

struct Stack<'a> {
    elements: Vec<Rc<object::Object<'a>>>,
}

impl<'a> Stack<'a> {
    fn top(&self) -> Option<Rc<object::Object<'a>>> {
        self.elements.last().map(|l| Rc::clone(l))
    }
    fn new() -> Self {
        Stack {
            elements: Vec::with_capacity(STACK_SIZE),
        }
    }
    fn push(&mut self, obj: Rc<object::Object<'a>>) {
        self.elements.push(obj);
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
            bytecode: bytecode,
            stack: Stack::new(),
        }
    }

    pub fn run(&mut self) {
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
                }
            } else {
                should_continue = false;
            }
        }
    }

    pub fn stack_top(&self) -> Option<Rc<object::Object<'ast>>> {
        self.stack.top()
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
        vm.run();
        let top = vm.stack_top();
        let obj_opt: Option<&object::Object> = top.as_ref().map(|a| &**a);
        assert_eq!(obj_opt, Some(&case.expected));
    }

    #[test]
    fn vm_tests() {
        let tests = vec![VmTestCase {
            input: "3 + 4",
            expected: object::Object::Integer(4),
        }];
        for test in tests {
            run_vm_test(test);
        }
    }
}
