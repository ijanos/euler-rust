pub fn main() {
    let pentagonals = (1..).map(|n| n * (3 * n - 1) / 2);
    let pentagonals = pentagonals.take(100_000).collect::<Vec<u64>>();
    'outer: for (i, &p) in pentagonals.iter().enumerate() {
        'inner: for k in (1..i).rev() {
            for j in (0..k).rev() {
                let sum = pentagonals[j] + pentagonals[k];
                if sum < p {
                    if j + 1 == k {
                        break 'inner;
                    } else {
                        continue 'inner;
                    }
                }
                if sum == p {
                    let d = pentagonals[k] - pentagonals[j];
                    if pentagonals.contains(&d) {
                        println!("{}", d);
                        break 'outer;
                    }
                }
            }
        }
    }
}
