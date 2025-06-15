struct Rpn {
    source: Vec<char>,
    stack: Vec<i16>,
}

impl Rpn {
    fn new(source: String) -> Self {
        let source = source
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<Vec<_>>();

        Rpn {
            source,
            stack: Vec::new(),
        }
    }

    fn eval(&mut self) -> i32 {
        0
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
}
