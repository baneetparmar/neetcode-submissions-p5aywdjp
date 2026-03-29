impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut maximum_profit_possible = 0;
        for right in 1..prices.len(){
            if prices[right] > prices[left]{
                let profit = prices[right] - prices[left];
                maximum_profit_possible = maximum_profit_possible.max(profit);
            } else {
                left = right;
            }
        }
        maximum_profit_possible
    }
}
