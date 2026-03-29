impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res:Vec<i32> = vec![0;temperatures.len()];
        let mut stack:Vec<usize> = vec![];

        for ti in 0..temperatures.len() {
            while let Some(&top) = stack.last() {
                if temperatures[top] < temperatures[ti] {
                    res[top] = (ti - top) as i32;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(ti);
        }
        res
    }
}
