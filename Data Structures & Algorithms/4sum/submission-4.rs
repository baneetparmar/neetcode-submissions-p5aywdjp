impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut sorted = nums;
        sorted.sort();
        
        let mut output: Vec<Vec<i32>> = vec![];
        let n = sorted.len();
        
        if n < 4 {
            return output;
        }
        
        let target = target as i64;
        
        for i in 0..(n - 3) {
            if i > 0 && sorted[i] == sorted[i - 1] {
                continue;
            }
            
            for j in (i + 1)..(n - 2) {
                if j > i + 1 && sorted[j] == sorted[j - 1] {
                    continue;
                }
                
                let inner_target = target - sorted[i] as i64 - sorted[j] as i64;
                let mut l = j + 1;
                let mut r = n - 1;
                
                while l < r {
                    let sum = sorted[l] as i64 + sorted[r] as i64;
                    if sum == inner_target {
                        output.push(vec![sorted[i], sorted[j], sorted[l], sorted[r]]);
                        
                        while l < r && sorted[l] == sorted[l + 1] {
                            l += 1;
                        }
                        while l < r && sorted[r] == sorted[r - 1] {
                            r -= 1;
                        }
                        
                        l += 1;
                        r -= 1;
                    } else if sum < inner_target {
                        l += 1;
                    } else {
                        r -= 1;
                    }
                }
            }
        }
        output
    }
}