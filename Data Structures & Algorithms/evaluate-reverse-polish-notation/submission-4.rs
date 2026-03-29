impl Solution {
    fn perform_operation(operand1:i32, operand2:i32, operator:&str) -> i32 {
        match operator {
            "+" => operand1 + operand2,
            "-" => operand1 - operand2,
            "*" => operand1 * operand2,
            "/" => operand1 / operand2,
            _ => unreachable!()
        }
    }

    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack:Vec<i32> = vec![];
        for ch in tokens {
            match ch.as_str() {
                "+" | "-" | "*" | "/" => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();
                    let res = Self::perform_operation(operand1,operand2,ch.as_str());
                    stack.push(res);
                },
                _ => {
                    let num = ch.parse::<i32>().unwrap();
                    stack.push(num);
                }
            }
        }
        stack.pop().unwrap()
    }
}