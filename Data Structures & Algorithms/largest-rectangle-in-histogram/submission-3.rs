impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut stack: Vec<(usize, i32)> = vec![];

        for (i, &height) in heights.iter().enumerate() {
            let mut start = i;
            while let Some(&(li, lh)) = stack.last() {
                if lh > height {
                    stack.pop();
                    max_area = max_area.max(lh * (i - li) as i32);
                    start = li;  // extend start back
                } else {
                    break;
                }
            }
            stack.push((start, height));
        }

        let len = heights.len();
        while let Some((i, h)) = stack.pop() {
            max_area = max_area.max(h * (len - i) as i32);
        }

        max_area
    }
}
