struct StockSpanner {
    day:usize,
    stack:Vec<(i32,usize)>,
}

impl StockSpanner {
    pub fn new() -> Self {
        Self {
            day:0,
            stack:vec![],
        }
    }

    pub fn next(&mut self, price: i32) -> i32 {
        let mut res = 0;
        self.day += 1;
        while let Some(&stock) = self.stack.last() {
            if stock.0 <= price {
                self.stack.pop();
            } else { break; }
        }
        let span = if let Some(&stock) = self.stack.last() {
            self.day - stock.1
        } else {
            self.day
        };
        self.stack.push((price,self.day));
        span as i32
    }
}
