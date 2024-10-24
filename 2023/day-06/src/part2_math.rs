/*!
# AoC 2023 Day 6 part 2
See the description of the puzzle at <https://adventofcode.com/2023/day/6>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

This puzzle is quite easy to solve with brute force. The more interesting part is
to make the code fast by using mathematics, which in my case is 100000 times faster!

The resulting race distance (d) is calculated as the (race time (t) minus the time for the button press (b))
times the resulting speed (s). Goal is to beat the current record (r).
d = (t - b) * s
S is calculated by the time of the button press and in this case B = S
d = (t - s) * s
Since the current record must be broken, the minimum result is > R.
r + 1 = (t - s) * s
or in example 30
200 = (30 - s) * s
This can be written as
201 = 30s - s^2
s^2 - 30s + 200 = 0
Which is the Quadratic Equation or pq formula x^2 + px + q = 0.
[Quadratic_equation](https://en.wikipedia.org/wiki/Quadratic_equation)
x = -(x/2) +/- SQRT((p/2)^2-q)
s = -(-30/2) +/- SQRT((30/2)^2-200)
s = 15 +/-

*/

/// Calculates the won races depending on button push time.
pub fn solve_puzzle(input: &str) -> String {
    // let races = input
    //     .lines()
    //     .map(|line| {
    //         let p = line.find(':').expect("data has wrong format") + 1;
    //         let mut n = 0;
    //         line[p..].chars().for_each(|c| {
    //             if c.is_ascii_digit() {
    //                 n = n * 10 + c.to_digit(10).unwrap() as u64;
    //             }
    //         });
    //         n
    //     })
    //     .collect::<Vec<_>>();
    type Fxx = f64;

    let mut race_time_ms = 0.0;
    let mut record_distance_mm = 0.0;

    let mut is_distance = false;
    let mut n = 0;
    for &c in input.as_bytes().iter().skip(12) {
        match c {
            b'0'..=b'9' => {
                n = n * 10 + (c - b'0') as u64;
            }
            b'\n' => {
                if is_distance {
                    record_distance_mm = n as Fxx;
                } else {
                    race_time_ms = n as Fxx;
                    is_distance = true;
                    n = 0;
                }
            }
            _ => {
                if is_distance {
                    record_distance_mm = n as Fxx;
                } else {
                    race_time_ms = n as Fxx;
                }
            }
        }
    }

    // this is actually only one race
    let rt2 = race_time_ms / 2.0;
    let sq = (rt2 * rt2 - record_distance_mm).sqrt();
    let r1 = rt2 - sq;
    let r2 = rt2 + sq;
    let speed1 = r1 as i64 + 1; // round down + 1
    let speed2 = r2.ceil() as i64 - 1; // round up + 1

    let winning_races_count = speed2 - speed1 + 1;

    winning_races_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("71503", solve_puzzle(input));
    }
}
