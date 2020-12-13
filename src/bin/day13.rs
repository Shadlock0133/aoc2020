use std::{
    num::ParseIntError,
    sync::atomic::{AtomicU64, Ordering},
};

fn main() {
    let input = std::fs::read_to_string("inputs/day13.txt").unwrap();
    let (timestamp, buses) = parse_input(&input);
    let res = check_1(timestamp, &buses);
    println!("Part 1 - Answer: {}", res);
    let res = check_2(100_000_000_000_000, &buses);
    println!("Part 2 - Answer: {}", res);
}

fn parse_bus(input: &str) -> Result<Option<u32>, ParseIntError> {
    if input == "x" {
        return Ok(None);
    }
    input.parse().map(Some)
}

fn parse_buses(input: &str) -> Vec<Option<u32>> {
    input
        .split(',')
        .map(parse_bus)
        .collect::<Result<_, _>>()
        .unwrap()
}

fn parse_input(input: &str) -> (u32, Vec<Option<u32>>) {
    let mut lines = input.lines().map(str::trim);
    let timestamp = lines.next().unwrap().parse().unwrap();
    let buses = parse_buses(lines.next().unwrap());
    (timestamp, buses)
}

fn check_1(timestamp: u32, buses: &[Option<u32>]) -> u32 {
    for i in timestamp.. {
        let bus = buses
            .iter()
            .filter_map(Option::as_ref)
            .find(|bus| i % **bus == 0);
        if let Some(bus) = bus {
            let time = i - timestamp;
            return bus * time;
        }
    }
    unreachable!()
}

fn check_2_fast(timestamp: u64, buses: &[Option<u32>]) -> u64 {
    static MIN: AtomicU64 = AtomicU64::new(u64::MAX);

    let max_bus = buses
        .iter()
        .enumerate()
        .filter_map(|(i, b)| Some((i, *b.as_ref()?)))
        .max_by_key(|x| x.1)
        .unwrap();

    let n = 12;
    let (s, r) = std::sync::mpsc::channel();
    let mut handles = vec![];
    MIN.store(u64::MAX, Ordering::SeqCst);
    for i in 0..n {
        let buses = buses.to_vec();
        let s = s.clone();
        let handle = std::thread::spawn(move || {
            let mut iter = (0u64..)
                .map(|x| x * n + i)
                .filter_map(|x| {
                    ((timestamp / max_bus.1 as u64 + x) * max_bus.1 as u64)
                        .checked_sub(max_bus.0 as u64)
                })
                .peekable();
            loop {
                for _ in 0..128 {
                    let time = iter.next().unwrap();
                    let check = buses
                        .iter()
                        .enumerate()
                        .filter_map(|(i, b)| Some((i, b.as_ref()?)))
                        .all(|(i, b)| (time + i as u64) % (*b as u64) == 0);
                    if check {
                        MIN.fetch_min(time, Ordering::SeqCst);
                        s.send(time).unwrap();
                        drop(s);
                        return;
                    }
                }
                if *iter.peek().unwrap() > MIN.load(Ordering::SeqCst) {
                    drop(s);
                    return;
                }
            }
        });
        handles.push(handle);
    }
    drop(s);
    for handle in handles {
        handle.join().unwrap();
    }
    r.iter().min().unwrap()
}

fn check_2_simple(timestamp: u64, buses: &[Option<u32>]) -> u64 {
    let max_bus = buses
        .iter()
        .enumerate()
        .filter_map(|(i, b)| Some((i, *b.as_ref()?)))
        .max_by_key(|x| x.1)
        .unwrap();
    (0u64..)
        .filter_map(|x| {
            ((timestamp / max_bus.1 as u64 + x) * max_bus.1 as u64).checked_sub(max_bus.0 as u64)
        })
        .find(|time| {
            buses
                .iter()
                .enumerate()
                .filter_map(|(i, b)| Some((i, b.as_ref()?)))
                .all(|(i, b)| (time + i as u64) % (*b as u64) == 0)
        })
        .unwrap()
}

fn check_2(timestamp: u64, buses: &[Option<u32>]) -> u64 {
    check_2_simple(timestamp, buses)
    // check_2_fast(timestamp, buses)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "939
        7,13,x,x,59,x,31,19";

    #[test]
    fn test1() {
        let output = 295;
        let (timestamp, buses) = parse_input(INPUT);
        let res = check_1(timestamp, &buses);
        assert_eq!(res, output);
    }

    #[test]
    fn test2() {
        let output = 1068781;
        let (_, buses) = parse_input(INPUT);
        let res = check_2(0, &buses);
        assert_eq!(res, output);

        let tests = [
            ("17,x,13,19", 3417),
            ("67,7,59,61", 754018),
            ("67,x,7,59,61", 779210),
            ("67,7,x,59,61", 1261476),
            ("1789,37,47,1889", 1202161486),
        ];
        for (input, output) in &tests {
            let buses = parse_buses(input);
            let res = check_2(0, &buses);
            assert_eq!(res, *output);
        }
        for (input, output) in &tests {
            let buses = parse_buses(input);
            let res = check_2(*output, &buses);
            assert_eq!(res, *output);
        }
    }
}
