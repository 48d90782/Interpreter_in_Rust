use crate::token::Token;

trait Node {
    fn token_literal(&mut self) -> String;
}

trait Statement {
    fn token_literal(&mut self) -> String;
    fn statement_node();
}

trait Expression {
    fn token_literal(&mut self) -> String;
    fn expression_node();
}

struct expression {}

impl Expression for expression {
    fn token_literal(&mut self) -> String {
        unimplemented!()
    }

    fn expression_node() {
        unimplemented!()
    }
}

type Stmt = Box<dyn Node>;

struct Program {
    statements: Vec<Stmt>,
}
struct Identifier {
    token: Token,
    value: String,
}

impl Node for Program {
    fn token_literal(&mut self) -> String {
        // len == 0
        if !self.statements.is_empty() {
            return self.statements[0].token_literal();
        }

        "\0".to_string()
    }
}

impl Expression for Identifier {
    fn token_literal(&mut self) -> String {
        unimplemented!()
    }

    fn expression_node() {
        unimplemented!()
    }
}

struct LetStatement {
    token: Token,
    name: Identifier,
    value: expression,
}

impl LetStatement {
    fn sll(&mut self) {
        self.value.to
    }
}
