use std::fs::read_to_string;
use hashbrown::HashMap;

fn reader(input: String) -> Vec<String> {
    read_to_string(input)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        _ if a == b => a,
        ((_, 0), _) => a,
        ((0, _), _) => b,
        (_, (0, 1) | (1, 0)) => gcd(a >> 1, b),
        (_, (0, 0)) => gcd(a >> 1, b >> 1) << 1,
        (_, (1, 1)) => {
            let (a, b) = (a.min(b), a.max(b));
            gcd((b - a) >> 1, a)
        }
        _ => unreachable!(),
    }
}

fn steps(path: &[u8], graph: &HashMap<&[u8],(&[u8],&[u8])>, start: &[u8], goal: &[u8]) -> usize {
    let mut node = start;
    1 + path.iter().cycle().position(|&d| {
        node = if d == b'L' {graph[node].0} else {graph[node].1};
        node.ends_with(goal)
    }).unwrap()
}

pub fn compute_1(input: String) -> usize {
    let result: usize;
    let contents: Vec<String> = reader(input);
    
    let path: &String = contents[0..2].first().unwrap();
    let rest: Vec<String> = contents[2..].to_vec();
    let graph = rest.iter().map(|l| {
        let l = l.as_bytes();
        (&l[0..3], (&l[7..10], &l[12..15]))
    }).collect::<HashMap<_,_>>();
    result = steps(path.as_bytes(), &graph, b"AAA", b"ZZZ");

    result
}

pub fn compute_2(input: String) -> usize {
    let result: usize;
    let contents: Vec<String> = reader(input);
    
    let path: &String = contents[0..2].first().unwrap();
    let rest: Vec<String> = contents[2..].to_vec();

    let graph = rest.iter().map(|l| {
        let l = l.as_bytes();
        (&l[0..3], (&l[7..10], &l[12..15]))
    }).collect::<HashMap<_,_>>();
    result = graph.keys()
        .filter(|k| k.ends_with(b"A"))
        .map(|node| steps(path.as_bytes(), &graph, node, b"Z"))
        .fold(1, |ans, x| (x*ans) / gcd(x,ans));

    result
}