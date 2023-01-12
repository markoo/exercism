#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    if inputs.is_empty() {
        return None;
    }

    let mut stack = Vec::new();

    for input in inputs {
        match input {
            CalculatorInput::Value(n) => stack.push(*n),
            _ => if stack.len() < 2 {
                return None
            } else {
                let b = stack.pop()?;
                let a = stack.pop()?;
                match input {
                    CalculatorInput::Add => stack.push(a + b),
                    CalculatorInput::Subtract => stack.push(a - b),
                    CalculatorInput::Multiply => stack.push(a * b),
                    CalculatorInput::Divide => stack.push(a / b),
                    _ => return None,
                }
            }
        }
    }
    match stack.len() {
        1 => Some(stack[0]),
        _ => None
    }
}
