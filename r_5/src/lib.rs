// Ma solution pour la partie 2 : ~30min
// Cette solution pour la partie 2 : ~3sec
// https://www.reddit.com/r/adventofcode/comments/18b4b0r/comment/kc2oxqe/

use std::cmp;
use std::collections::VecDeque;

type MapRange = (usize, usize, usize);
type SeedInterval = (usize, usize);
type MapInterval = (usize, usize, usize);

fn convert(source_num: usize, map: &Vec<MapRange>) -> usize {
    for (dest_start, src_start, len) in map {
        if *src_start <= source_num && source_num < src_start + len {
            return (source_num - src_start) + dest_start;
        }
    }
    source_num
}

fn intersect(seed: &SeedInterval, map: &MapInterval) -> Option<SeedInterval> {
    if seed.1 <= map.1 {
        return None;
    } else if seed.0 >= map.2 {
        return None;
    } else {
        let max_start = cmp::max(seed.0, map.1);
        let min_end = cmp::min(seed.1, map.2);
        return Some((max_start, min_end));
    }
}

fn convert_2(seed: &SeedInterval, sorted_map: &Vec<MapInterval>) -> Vec<SeedInterval> {
    let mut seeds = VecDeque::from([seed.to_owned()]);
    let mut dests = Vec::from([]);

    while !seeds.is_empty() {
        let src = seeds.pop_front().expect("Deque should be non-empty");
        let mut intersected = false;

        for map_interval in sorted_map {
            if let Some(intersection) = intersect(&src, map_interval) {
                if src.0 < intersection.0 {
                    seeds.push_back((src.0, intersection.0 - 1));
                }
                if intersection.1 < src.1 {
                    seeds.push_back((intersection.1, src.1));
                }
                dests.push((
                    map_interval.0 + (intersection.0 - map_interval.1),
                    map_interval.0 + (intersection.1 - map_interval.1),
                ));
                intersected = true;
                break;
            }
        }
        if !intersected {
            dests.push(src);
        }
    }

    dests
}

fn parse_map_line(line: &str) -> MapRange {
    let nums: Vec<_> = line
        .split_whitespace()
        .map(|w| w.parse().expect("Map range should be a number"))
        .collect();
    (nums[0], nums[1], nums[2])
}

fn parse_seeds(line: &str) -> Vec<usize> {
    line.split(": ").collect::<Vec<_>>()[1]
        .split_whitespace()
        .map(|w| w.parse().expect("Seed should be a number"))
        .collect()
}

fn parse_seeds_2(line: &str) -> Vec<(usize, usize)> {
    let seed_nums = line.split(": ").collect::<Vec<_>>()[1]
        .split_whitespace()
        .map(|w| w.parse().expect("Seed range should be a number"))
        .collect::<Vec<_>>();
    let mut seed_ranges = Vec::new();

    let mut i = 0;
    while i < seed_nums.len() {
        seed_ranges.push((seed_nums[i], seed_nums[i] + seed_nums[i + 1]));
        i += 2;
    }

    seed_ranges
}

fn parse(input: &str) -> (Vec<usize>, Vec<Vec<MapRange>>) {
    let lines: Vec<_> = input.lines().collect();
    let seeds = parse_seeds(lines[0]);
    let mut maps = Vec::new();

    let mut i = 1;
    while i < lines.len() {
        let line = lines[i];

        if line.is_empty() {
            i += 1;
            continue;
        }

        if line.contains("map") {
            let mut map = Vec::new();
            i += 1;

            while i < lines.len() && !lines[i].is_empty() {
                map.push(parse_map_line(lines[i]));
                i += 1;
            }
            maps.push(map);
        }

        i += 1;
    }

    (seeds, maps)
}

fn parse_2(input: &str) -> (Vec<SeedInterval>, Vec<Vec<MapInterval>>) {
    let lines: Vec<_> = input.lines().collect();
    let seeds = parse_seeds_2(lines[0]);
    let mut maps = Vec::new();

    let mut i = 1;
    while i < lines.len() {
        let line = lines[i];

        if line.is_empty() {
            i += 1;
            continue;
        }

        if line.contains("map") {
            let mut map = Vec::new();
            i += 1;

            while i < lines.len() && !lines[i].is_empty() {
                let parsed_line = parse_map_line(lines[i]);
                map.push((parsed_line.0, parsed_line.1, parsed_line.1 + parsed_line.2));
                i += 1;
            }
            maps.push(map);
        }

        i += 1;
    }

    (seeds, maps)
}

fn sort_range(a: &MapRange, b: &MapRange) -> cmp::Ordering {
    a.1.cmp(&b.1)
}

pub fn part_1(input: &str) -> usize {
    let (seeds, maps) = parse(input);

    let mut min_loc = usize::MAX;
    for seed in seeds {
        let mut prev_loc = seed;
        let mut curr_loc = 0;
        for map in &maps {
            curr_loc = convert(prev_loc, map);
            prev_loc = curr_loc;
        }
        min_loc = cmp::min(min_loc, curr_loc);
    }

    min_loc
}

pub fn part_2(input: &str) -> usize {
    let (seeds, maps) = parse_2(input);
    let mut maps = maps;
    for map in maps.iter_mut() {
        map.sort_by(|a, b| sort_range(a, b));
    }

    let mut src_intervals = seeds;
    for map in &maps {
        let mut dst_intervals = Vec::new();
        for seed_interval in &src_intervals {
            dst_intervals.append(&mut convert_2(seed_interval, map));
        }
        src_intervals = dst_intervals;
    }
    let result = src_intervals
        .iter()
        .min_by_key(|i| i.0)
        .expect("Should have a min interval");

    result.0
}
