pub struct Solution;

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let mut k  = k as i64;
        let mut output = 0;
        let sum = chalk.iter().fold(0_i64, |acc, x| &acc + (*x as i64));
        k %= sum;
        (0..chalk.len()).for_each(|_| {
            if k - chalk[output] as i64 >= 0 {
                k -= chalk[output] as i64;
                output += 1;
            } 
        });
        output as i32
    }
}