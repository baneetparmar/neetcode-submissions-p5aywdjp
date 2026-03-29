impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack:Vec<i32> = vec![];
        for ch in tokens {
            match ch.as_str() {
                operator @ ("+" | "-" | "*" | "/") => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();
                    let res = match operator {
                        "+" => operand1 + operand2,
                        "-" => operand1 - operand2,
                        "*" => operand1 * operand2,
                        "/" => operand1 / operand2,
                        _ => unreachable!()
                    };
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