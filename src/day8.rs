use std::collections::{HashSet, BinaryHeap};
use std::cmp::Ordering;

type Point = (usize, usize, usize);
type Pairs = BinaryHeap<Pair>;
type Input = (Vec<Point>, Pairs);
type Set = HashSet<Point>;

#[derive(Eq, PartialEq, Clone)]
pub struct Pair {
    a: Point,
    b: Point,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_dist = (self.a.0 - self.b.0).pow(2) + (self.a.1 - self.b.1).pow(2) + (self.a.2 - self.b.2).pow(2);
        let other_dist = (other.a.0 - other.b.0).pow(2) + (other.a.1 - other.b.1).pow(2) + (other.a.2 - other.b.2).pow(2);
        other_dist.cmp(&self_dist)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Input {
    let points: Vec<Point> = input.lines().map(|l| {
        let mut nums = l.split(',');
        (
            nums.nth(0).unwrap().parse::<usize>().unwrap(),
            nums.nth(0).unwrap().parse::<usize>().unwrap(),
            nums.nth(0).unwrap().parse::<usize>().unwrap()
        )
    }).collect();
    let mut pairs: BinaryHeap<Pair> = BinaryHeap::new();
    points
        .iter()
        .enumerate()
        .for_each(|(i, a)| {
            points.iter().skip(i + 1).for_each(|b| {
                pairs.push(Pair {
                    a: *a,
                    b: *b
                });
            });
        });
    (points, pairs)
}

fn do_joins((points, pairs): &Input) -> (usize, usize) {
    let mut pairs = pairs.clone();
    let mut sets: Vec<Set> = vec![];
    for point in points {
        let mut set = HashSet::new();
        set.insert(*point);
        sets.push(set);
    }
    let mut sets_at_thousand: Option<Vec<Set>> = None;
    let mut last_joined_pair: Option<Pair> = None;
    let mut count = 0;
    while let Some(pair) = pairs.pop() {
        if count == 1000 {
            let mut sets_clone = sets.clone();
            sets_clone.sort_by(|a, b| b.len().cmp(&a.len()));
            sets_at_thousand = Some(sets_clone);
        }
        count += 1;
        let a_set_idx = sets.iter().position(|s| s.contains(&pair.a)).unwrap();
        let b_set_idx = sets.iter().position(|s| s.contains(&pair.b)).unwrap();
        if a_set_idx == b_set_idx {
            continue;
        }
        last_joined_pair = Some(pair);
        let b_points: Vec<Point> = sets[b_set_idx].iter().cloned().collect();
        for p in b_points {
            sets[b_set_idx].remove(&p);
            sets[a_set_idx].insert(p);
        }
    }
    let sets_at_thousand = sets_at_thousand.unwrap();
    let last_joined_pair = last_joined_pair.unwrap();
    (
        sets_at_thousand[0].len() * sets_at_thousand[1].len() * sets_at_thousand[2].len(),
        last_joined_pair.a.0 * last_joined_pair.b.0
    )
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Input) -> usize {
    do_joins(input).0
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Input) -> usize {
    do_joins(input).1
}
