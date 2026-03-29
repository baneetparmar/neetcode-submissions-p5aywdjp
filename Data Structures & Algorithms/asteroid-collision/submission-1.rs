impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack:Vec<i32> = vec![];

        for asteroid in asteroids {
            if stack.is_empty() || asteroid > -1 {
                stack.push(asteroid);
            } else {
                while !stack.is_empty() && 
                    *stack.last().unwrap() > -1 &&
                    *stack.last().unwrap() < asteroid.abs(){
                        stack.pop();
                };
                if !stack.is_empty() {
                    if *stack.last().unwrap() == asteroid.abs(){
                        stack.pop();
                        continue;
                    } else if *stack.last().unwrap() > asteroid.abs(){
                        continue;
                    }
                }
                stack.push(asteroid);
            }
        }
        stack
    }
}
