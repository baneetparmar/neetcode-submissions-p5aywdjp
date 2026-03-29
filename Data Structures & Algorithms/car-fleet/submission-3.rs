impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut paired:Vec<(i32,i32)> = position.iter().zip(speed.iter()).map(|(&p,&s)| (p,s)).collect();
        paired.sort_by_key(|pair| pair.0);
        paired.reverse();

        let mut fleets = vec![];

        for (pos,spd) in paired {
            let time = (target - pos) as f64 / spd as f64;
            if fleets.is_empty() || *fleets.last().unwrap() < time {
                fleets.push(time);
            }
        }
        fleets.len() as i32

    }
}
