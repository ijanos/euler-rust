// Create a two dimensional array, every position contains the number
// of possible routes.
//
// 1  1  1  1
// 1  2  3  4
// 1  3  6 10
// 1  4 10 20
//
// The answer is in row 20 column 20, starting with zero indexing.

pub fn main() {
    let mut lattice: [[u64; 21]; 21] = [[0; 21]; 21];

    // initialize first row and column to 1s
    lattice[0] = [1; 21];
    for y in 0..21  {
        lattice[y][0] = 1;
    }

    for x in 1..21 {
        for y in 1..21 {
            lattice[x][y] = lattice[x-1][y] + lattice[x][y-1];
        }
    }
    println!("{}", lattice[20][20]);
}