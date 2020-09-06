use crate::{ast, code, object};
use std::convert::TryInto;

#[derive(Debug)]
enum AstNode<'a> {
    Program(&'a ast::Program),
    Statement(&'a ast::Statement),
    Expression(&'a ast::Expression),
}

struct Compiler<'a> {
    instructions: Vec<u8>,
    constants: Vec<object::Object<'a>>,
}

impl<'a> Compiler<'a> {
    fn new() -> Self {
        Compiler {
            instructions: vec![],
            constants: vec![],
        }
    }
    fn add_constant(&mut self, obj: object::Object<'a>) {
        let next_const_index = self.constants.len();
        let next_const_index: u16 = next_const_index.try_into().unwrap();
        self.constants.push(obj);
        Vec::append(
            &mut self.instructions,
            &mut code::Instruction::Constant(next_const_index).to_bytes(),
        );
    }
    fn compile(&mut self, node: AstNode) {
        match node {
            AstNode::Expression(expression) => match expression {
                ast::Expression::IntegerLiteral { value } => {
                    self.add_constant(object::Object::Integer(*value));
                }
                ast::Expression::Prefix { right, .. } => {
                    self.compile(AstNode::Expression(&right));
                }
                ast::Expression::Infix { left, right, .. } => {
                    self.compile(AstNode::Expression(&left));
                    self.compile(AstNode::Expression(&right));
                }
                ast::Expression::Block { statements } => {
                    for statement in statements {
                        self.compile(AstNode::Statement(&statement));
                    }
                }
                _ => {}
            },
            AstNode::Program(program) => {
                for statement in &program.statements {
                    self.compile(AstNode::Statement(&statement));
                }
            }
            AstNode::Statement(statement) => {
                match statement {
                    ast::Statement::Let { .. } => {
                        //
                    }
                    ast::Statement::Return { .. } => {
                        //
                    }
                    ast::Statement::Expression { expression } => {
                        self.compile(AstNode::Expression(expression))
                    }
                }
            }
        }
    }

    fn bytecode(self) -> Bytecode<'a> {
        Bytecode {
            instructions: self.instructions,
            constants: self.constants,
        }
    }
}

struct Bytecode<'a> {
    instructions: Vec<u8>,
    constants: Vec<object::Object<'a>>,
}

#[cfg(test)]
mod test {
    use crate::{ast, code, compiler, lexer, object, parser};
    struct CompilerTestCase<'a> {
        input: &'static str,
        expected_constants: Vec<object::Object<'a>>,
        expected_instructions: Vec<Vec<u8>>,
    }

    #[test]
    fn test_integer_arithmetic() {
        let tests: Vec<CompilerTestCase> = vec![CompilerTestCase {
            input: "1 + 2",
            expected_instructions: vec![
                code::Instruction::Constant(0).to_bytes(),
                code::Instruction::Constant(1).to_bytes(),
            ],
            expected_constants: vec![object::Object::Integer(1), object::Object::Integer(2)],
        }];
        for test in tests {
            run_compiler_test(test);
        }
    }

    fn parse(input: &'static str) -> ast::Program {
        let mut lexer = lexer::new(input);
        let mut parser = parser::Parser::new(&mut lexer);
        parser.parse_program().unwrap()
    }

    fn run_compiler_test(test: CompilerTestCase) {
        let program = parse(test.input);
        let mut compiler = compiler::Compiler::new();
        compiler.compile(compiler::AstNode::Program(&program));
        let bytecode = compiler.bytecode();
        let expected_instructions_bytecode = test
            .expected_instructions
            .into_iter()
            .flat_map(|i| i)
            .collect::<Vec<u8>>();
        assert_eq!(expected_instructions_bytecode, bytecode.instructions);
        assert_eq!(
            test.expected_constants,
            vec![object::Object::Integer(1), object::Object::Integer(2)]
        )
    }
}
