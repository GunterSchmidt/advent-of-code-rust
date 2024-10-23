/*!
# AoC 2023 Day 5 part 1
See the description of the puzzle at <https://adventofcode.com/2023/day/5>.\
Many thanks to Eric Wastl for providing these challenges.

MIT License\
Copyright (c) 2024 Gunter Schmidt.\
Source Code: <https://github.com/GunterSchmidt/advent-of-code-rust>

---
**Coding Highlights**

Just following the data stream.

*/

#[derive(Debug)]
struct SeedMap {
    seeds: Vec<u64>,
    mappings: Vec<Mapping>,
}

impl SeedMap {
    fn get_location_from_seed(&self, seed: u64) -> u64 {
        let mut dest = seed as i64;
        'for_map: for mapping in &self.mappings {
            let mut last = mapping.mapping_data.len() - 1;
            let mut check_idx = 0;
            let mut smaller_idx = 0;
            if last > 3 {
                // make a binary search until source is < then source_from and the jump is small
                check_idx = last / 2;
                loop {
                    if dest > mapping.mapping_data[check_idx].source_to {
                        // not yet far enough
                        let next_idx = (last + check_idx) / 2;
                        if next_idx - check_idx < 3 {
                            check_idx += 1;
                            break;
                        }
                        // set smaller index to following element as this was already checked too small
                        smaller_idx = check_idx + 1;
                        check_idx = next_idx;
                    } else {
                        // maybe too far
                        if dest >= mapping.mapping_data[check_idx].source_from {
                            // found
                            dest = dest + mapping.mapping_data[check_idx].to_dest_add;
                            continue 'for_map;
                        }
                        // too far
                        last = check_idx - 1;
                        if last - smaller_idx < 3 {
                            check_idx = smaller_idx;
                            break;
                        }
                        check_idx = (smaller_idx + last) / 2;
                        // dbg!(seed, source, check_idx, &mapping._from_name);
                    }
                }
            }

            loop {
                let mapping_data = &mapping.mapping_data[check_idx];
                if dest >= mapping_data.source_from {
                    if dest <= mapping_data.source_to {
                        dest += mapping_data.to_dest_add;
                        break;
                    }
                } else {
                    // ordered, so no need to check the rest
                    // in this case source remains unchanged
                    break;
                }
                check_idx += 1;
                if check_idx > last {
                    break;
                }
            }
        }
        dest as u64
    }
}

#[derive(Debug)]
struct Mapping {
    // names only for debug
    _from_name: String,
    _to_name: String,
    mapping_data: Vec<MappingData>,
}

impl Mapping {
    fn new(from: &str, to: &str) -> Self {
        Self {
            _from_name: from.to_string(),
            _to_name: to.to_string(),
            mapping_data: Vec::new(),
        }
    }
}

// mapping gives a range and a delta to ease calculations
#[derive(Debug)]
struct MappingData {
    source_from: i64,
    source_to: i64,
    to_dest_add: i64,
}

// This is the main function.
pub fn solve_puzzle(input: &str) -> String {
    let seed_map = parse_data(input).expect("parse failed");
    let mut min_location = u64::MAX;
    for seed in &seed_map.seeds {
        let location = seed_map.get_location_from_seed(*seed);
        min_location = min_location.min(location);
    }

    min_location.to_string()
}

/// Parsing the data into a SeedMap
fn parse_data(input: &str) -> Option<SeedMap> {
    let mut seed_map = SeedMap {
        seeds: Vec::new(),
        mappings: Vec::with_capacity(7),
    };

    let mut colon_count = 0;
    let mut is_mapping_data = false;
    let mut mapping: Mapping = Mapping::new("", "");
    for line in input.lines() {
        if is_mapping_data {
            if line.len() < 5 {
                seed_map.mappings.push(mapping);
                mapping = Mapping::new("", "");
                is_mapping_data = false;
            } else {
                let m = line.split_whitespace().collect::<Vec<&str>>();
                let source_from = m[1].parse().unwrap();
                let mapping_data = MappingData {
                    source_from,
                    source_to: m[2].parse::<i64>().unwrap() + source_from - 1,
                    to_dest_add: m[0].parse::<i64>().unwrap() - source_from,
                };
                mapping.mapping_data.push(mapping_data);
            }
        } else {
            if colon_count == 0 {
                if line.starts_with("seeds: ") {
                    colon_count = 1;
                    let seeds: Vec<u64> = line[7..]
                        .split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect();
                    seed_map.seeds = seeds;
                }
            } else {
                if let Some(pos) = line.find(" map:") {
                    // this is the next map in order
                    let m = line[0..pos].split('-').collect::<Vec<&str>>();
                    mapping = Mapping::new(m[0], m[2]);
                    colon_count += 1;
                    is_mapping_data = true;
                }
            }
        }
    }
    seed_map.mappings.push(mapping);

    // sort the mappings by source_from
    for mapping in &mut seed_map.mappings {
        mapping
            .mapping_data
            .sort_by(|a, b| a.source_from.cmp(&b.source_from));
    }

    // dbg!(&seed_map);
    if seed_map.mappings.len() == 7 {
        Some(seed_map)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        assert_eq!("35", solve_puzzle(input));
    }
}
