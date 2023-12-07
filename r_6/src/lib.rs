use itertools::{izip, Itertools};
use std::fs::read_to_string;

#[derive(Debug)]
struct Run {
    time: usize,
    distance: usize,
}

impl Run {
    fn new(t_d: (&usize, &usize)) -> Self {
        Self {
            time: *t_d.0,
            distance: *t_d.1,
        }
    }

    fn compute(&self) -> usize {

        let a = (self.time - f64::sqrt((self.time * self.time - 4 * self.distance) as f64) as usize) / 2;
        let b = self.time as usize - a;
        b as usize - (b * (self.time - b) <= self.distance) as usize - a as usize - (a * (self.time - a) <= self.distance) as usize + 1
    }
}

#[derive(Debug)]
struct RunCollection {
    runs: Vec<Run>,
}

#[derive(Debug)]
struct OneRun {
    runs: Vec<Run>,
}

impl RunCollection {
    fn new(lines: Vec<String>) -> Self {
        let (_, lists) = lines[0].split_once(':').unwrap();
        let times = lists
            .split(' ')
            .filter(|p| p != &"")
            .filter_map(|word| word.parse::<usize>().ok());
        let (_, lists) = lines[1].split_once(':').unwrap();
        let distances = lists
            .split(' ')
            .filter(|p| p != &"")
            .filter_map(|word| word.parse::<usize>().ok());
        Self {
            runs: izip!(&times.collect_vec(), &distances.collect_vec())
                .map(Run::new)
                .collect::<Vec<Run>>(),
        }
    }
}

impl OneRun {
    fn new(lines: Vec<String>) -> Self {
        let line = lines[0].replace(" ", "");
        let (_, number) = line.split_once(':').unwrap();
        let time = number
            .parse::<usize>().unwrap();
        let line = lines[1].replace(" ", "");
        let (_, number) = line.split_once(':').unwrap();
        let distance = number
            .parse::<usize>().unwrap();
        Self {
            runs: vec![Run {
                time: time,
                distance: distance
            }],
        }
    }
}

fn reader(input: String) -> Vec<String> {
    read_to_string(input)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

pub fn compute_1(input: String) -> usize {
    let mut result: Vec<usize> = vec![];
    let contents: Vec<String> = reader(input);
    let run_collection: RunCollection = RunCollection::new(contents);

    for run in run_collection.runs {
        result.push(run.compute());
    }

    result.iter().product::<usize>() 

}

pub fn compute_2(input: String) -> usize {
    let mut result: Vec<usize> = vec![];
    let contents: Vec<String> = reader(input);
    let one_run: OneRun = OneRun::new(contents);
    
    for run in one_run.runs {
        result.push(run.compute());
    }

    result.iter().product::<usize>() 
}