#[derive(Debug)]
enum Operand {
    Number(i32),
    Variable
}

#[derive(Debug)]
enum Operation {
    Plus,
    Multiply
}

#[derive(Debug)]
pub struct MonkeyOperation {
    operation: Operation,
    right_hand: Operand
}

impl MonkeyOperation {
    pub fn from(monkey_expression: &str) -> Self {
        const EXPRESSION_START: &str = "  Operation: new = ";
        if !monkey_expression.starts_with(EXPRESSION_START) {
            panic!("Unexpected monkey expression")
        }
        let monkey_expression = &monkey_expression[EXPRESSION_START.len()..monkey_expression.len()];
        let mut expression_parts = monkey_expression.split(" ");
        expression_parts.next().expect("No left hand side");
        let operation = expression_parts.next().expect("No operation");
        let rhs = expression_parts.next().expect("No right hand side");

        let operation = match operation {
            "+" => Operation::Plus,
            "*" => Operation::Multiply,
            _ => panic!("Unknown operation")
        };

        let rhs = match rhs {
            "old" => Operand::Variable,
            x => {
                let number: i32 = x.parse().expect("No number at lhs");

                Operand::Number(number)
            }
        };

        MonkeyOperation { operation, right_hand: rhs }
    }

    pub fn evaluate(&self, old: i32) -> i32 {
        let rhs = match self.right_hand {
            Operand::Number(x) => x,
            Operand::Variable => old
        };

        match self.operation {
            Operation::Plus => old + rhs,
            Operation::Multiply => old * rhs
        }
    }
}