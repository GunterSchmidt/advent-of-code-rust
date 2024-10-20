/*!
# AoC 2023 Day 6 part 1
See the description of the puzzle at <https://adventofcode.com/2023/day/6>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

This puzzle is quite easy to solve with brute force. The more interesting part is
to make the code fast by using mathematics, which is just twice as fast for part 1.

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
    // parse data, time is first line, record distance is second line
    let races = input
        .lines()
        .map(|line| {
            let p = line.find(':').expect("data has wrong format") + 1;
            let mut data: Vec<u64> = Vec::with_capacity(4);
            // avoid string manipulation
            let mut n = 0;
            line[p..].chars().for_each(|c| {
                if c.is_ascii_digit() {
                    n = n * 10 + c.to_digit(10).unwrap() as u64;
                } else if n > 0 {
                    data.push(n);
                    n = 0;
                }
            });
            if n > 0 {
                data.push(n);
            }

            data
        })
        .collect::<Vec<_>>();
    // dbg!(&races);

    let mut winning_races_count = 1;
    for (i, race_time_ms) in races[0].iter().enumerate() {
        // let race_time_ms = races[0][i] as f64;
        let record_distance_mm = races[1][i] as f64;
        let rt2 = *race_time_ms as f64 / 2.0;
        let sq = (rt2 * rt2 - record_distance_mm).sqrt();
        let r1 = rt2 - sq;
        let r2 = rt2 + sq;
        let speed1 = r1 as i64 + 1; // round down + 1
        let speed2 = r2.ceil() as i64 - 1; // round up + 1
        winning_races_count *= speed2 - speed1 + 1;
    }

    winning_races_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("71503", solve_puzzle(input));
    }
}
