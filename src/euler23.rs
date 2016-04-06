pub fn main() {
    let divisor_sum = |n| (1..(n / 2 + 1)).filter(|&i| n % i == 0).fold(0, |sum, i| sum + i);
    let abundants = (1..).filter(|&i| divisor_sum(i) > i);
    let abundants = abundants.take_while(|&i| i < 27_000).collect::<Vec<_>>();

    let mut nums = [true; 29_000];
    for (i, x) in abundants.iter().enumerate() {
        for &y in &abundants[i..] {
            let s = x + y;
            if s < 29_000 {
                nums[s as usize] = false;
            }
        }
    }
    let ans = nums.iter().enumerate().filter(|&(_, &x)| x).fold(0, |sum, (i, _)| sum + i);
    println!("{:?}", ans);
}
