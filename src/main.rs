use leetcode::Solution;

fn main() {
    let nums = vec![3,4,1,2];
    println!("=> {:?}", Solution::chalk_replacer(nums, 25));
    let nums = vec![5,1,5];
    println!("=> {:?}", Solution::chalk_replacer(nums, 22));
}
