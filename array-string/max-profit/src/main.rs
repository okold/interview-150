fn main() {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut abs_max: i32 = 0;

        let mut max = prices[0];
        let mut min = prices[0];

        for val in prices {
            if val < min {

                let calc = max - min;
                if abs_max < calc {
                    abs_max = calc;
                }

                min = val;
                max = val;
            }
            else if val > max {
                max = val;
            }
        }

        let calc = max - min;
        if abs_max < calc {
            abs_max = calc;
        }

        return abs_max;
    }

    assert_eq!(5, max_profit([7,1,5,3,6,4].to_vec()));
    assert_eq!(0, max_profit([7,6,4,3,1].to_vec()));

}
