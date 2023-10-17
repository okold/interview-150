fn main() {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut total: i32 = 0;
        let mut max = prices[0];
        let mut min = prices[0];

        for val in prices {

            if val < max {
                // gather max profit of last window
                total += max - min;
                min = val;
                max = val;
            }
            else {
                max = val;
            }
        }

        total += max - min;

        return total;
    }

    assert_eq!(7, max_profit([7,1,5,3,6,4].to_vec()));
    assert_eq!(4, max_profit([1,2,3,4,5].to_vec()));
    assert_eq!(0, max_profit([7,6,4,3,1].to_vec()));
}
