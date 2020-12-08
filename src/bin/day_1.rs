const INPUT: &str = include_str!("../../input/day_1.txt");

fn main() {
    let mut codes: Vec<i32> = INPUT
        .lines()
        .filter_map(|l| l.trim().parse().ok())
        .collect();

    codes.sort_unstable();

    // product of two values that sum to 2020
    'two_outer: for (n, i) in codes.iter().enumerate() {
        for j in codes.iter().skip(n + 1) {
            if i + j == 2020 {
                println!("two values {} * {} = {}", i, j, i * j);
                break 'two_outer;
            }
        }
    }

    // product of three values that sum to 2020
    'three_outer: for (n, i) in codes.iter().enumerate() {
        for j in codes.iter().skip(n + 1) {
            for k in codes.iter().skip(n + 2) {
                if i + j + k == 2020 {
                    println!("three values {} * {} * {} = {}", i, j, k, i * j * k);
                    break 'three_outer;
                }
            }
        }
    }
}
