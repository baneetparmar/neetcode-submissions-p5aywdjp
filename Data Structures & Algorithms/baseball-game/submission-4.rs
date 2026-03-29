impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut stack:Vec<i32> = vec![];
        for st in operations {
            match st.as_str() {
                "+" => {
                    let op2 = stack.pop().unwrap();
                    let op1 = stack.pop().unwrap();
                    let res = op1 + op2;
                    stack.push(op1);
                    stack.push(op2);
                    stack.push(res);
                },
                "C" => {
                    stack.pop();
                },
                "D" => {
                    let op = *stack.last().unwrap();
                    let res = op * 2;
                    stack.push(res);
                },
                _ => {
                    let num:i32 = st.parse().unwrap();
                    stack.push(num);
                }
            }
        }

        stack.iter().sum()
    }
}
