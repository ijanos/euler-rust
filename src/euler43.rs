use permutohedron::Heap as Permutations;

fn  array2num(numbers: &[u8]) -> u64 {
    numbers.iter()
           .rev()
           .enumerate()
           .fold(0, |acc, (i, &n)| acc + 10_u64.pow(i as u32) * n as u64)
}

fn divisbility(numbers: &[u8; 10]) -> bool {
    array2num(&numbers[1..4])  % 2  == 0 &&
    array2num(&numbers[2..5])  % 3  == 0 &&
    array2num(&numbers[3..6])  % 5  == 0 &&
    array2num(&numbers[4..7])  % 7  == 0 &&
    array2num(&numbers[5..8])  % 11 == 0 &&
    array2num(&numbers[6..9])  % 13 == 0 &&
    array2num(&numbers[7..10]) % 17 == 0
}

pub fn main() {
    let mut input = [0,1,2,3,4,5,6,7,8,9];
    let perms = Permutations::new(&mut input);
    println!("{}", perms.filter(|p| divisbility(p)).fold(0, |acc, p| array2num(&p) + acc));
}
