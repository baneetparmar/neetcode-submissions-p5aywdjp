impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];

        for asteroid in asteroids {
            let mut destroyed = false;

            while let Some(&top) = stack.last(){
                // already passed the collision point or both going same direction
                if asteroid > 0 || top < 0 { 
                    break;
                } else if top == asteroid.abs() {
                    stack.pop();
                    destroyed = true;
                    break;
                } else if top < asteroid.abs() {
                    stack.pop();
                } else {
                    destroyed = true;
                    break;
                }
            }
            if !destroyed {
                stack.push(asteroid);
            }
        }
        stack
    }
}
