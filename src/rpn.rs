struct Rpn {
    source: Vec<String>,
    stack: Vec<i16>,
}

impl Rpn {
    fn new(source: String) -> Self {
        let source = source.split_whitespace().map(String::from).collect();

        Rpn {
            source,
            stack: Vec::new(),
        }
    }

    fn eval(&mut self) -> i32 {
        for c in &self.source {
            if c.chars().all(char::is_numeric) {
                let num: i16 = c.parse().expect("Failed to parse number");
                self.stack.push(num);
            } else {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");

                match c.chars().next().expect("Empty operator") {
                    '+' => self.stack.push(a + b),
                    '-' => self.stack.push(a - b),
                    '*' => self.stack.push(a * b),
                    '/' => {
                        if b == 0 {
                            panic!("Division by zero");
                        }
                        self.stack.push(a / b);
                    }
                    _ => panic!("Unknown operator: {}", c),
                }
            }
        }

        self.stack.pop().expect("No result on stack") as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trivial_case() {
        let input = "0";
        let mut rpn = Rpn::new(input.to_string());
        assert_eq!(rpn.eval(), 0);
    }

    #[test]
    fn simple_addition() {
        let input = "1 2 +";
        let mut rpn = Rpn::new(input.to_string());
        assert_eq!(rpn.eval(), 3);
    }

    #[test]
    fn complex_evaluation() {
        let input = "3 4 + 2 * 7 /";
        let mut rpn = Rpn::new(input.to_string());
        assert_eq!(rpn.eval(), 2);
    }

    #[test]
    fn multiple_digits() {
        let input = "12 34 +";
        let mut rpn = Rpn::new(input.to_string());
        assert_eq!(rpn.eval(), 46);
    }
}
