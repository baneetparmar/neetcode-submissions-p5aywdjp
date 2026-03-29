impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut boats = 0;
        let mut people = people;
        people.sort();
        let mut l = 0i32;
        let mut r = (people.len() - 1) as i32;
        
        while l <= r {
            if people[l as usize] + people[r as usize] <= limit {
                l += 1;
            }
            r -= 1;
            boats += 1;
        }
        boats
    }
}