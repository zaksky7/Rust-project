use std::cmp::{max, min};
use std::collections::HashMap;

use crate::year2016::day10::Src::*;

enum Src {
    Value(i64),
    Bot(String, fn(i64, i64) -> i64),
}

type Node = Vec<Src>;

fn populate_ins(m: &mut HashMap<String, Vec<i64>>, tbl: &HashMap<String, Node>, k: &String) {
    if m.contains_key(k) {
        return;
    }
    let mut inps: Vec<i64> = tbl[k]
        .iter()
        .map(|src| match src {
            Value(v) => *v,
            Bot(b, f) => {
                populate_ins(m, tbl, b);
                m[b].iter().copied().reduce(f).unwrap()
            }
        })
        .collect();
    inps.sort();
    m.insert(k.clone(), inps);
}

fn run_factory(input: &str) -> HashMap<String, Vec<i64>> {
    let mut tbl: HashMap<String, Node> = HashMap::new();
    for line in input.lines() {
        match line.split_whitespace().collect::<Vec<_>>()[..] {
            ["bot", n, _, _, _, o1, n1, _, _, _, o2, n2] => {
                let e = tbl.entry(format!("{} {}", o1, n1)).or_insert(vec![]);
                e.push(Bot(format!("bot {}", n), min));
                let e = tbl.entry(format!("{} {}", o2, n2)).or_insert(vec![]);
                e.push(Bot(format!("bot {}", n), max));
            }
            ["value", v, _, _, o, n] => {
                let e = tbl.entry(format!("{} {}", o, n)).or_insert(vec![]);
                e.push(Value(v.parse().unwrap()));
            }
            _ => panic!("Parse failed: {}", line),
        }
    }
    let mut result: HashMap<String, Vec<i64>> = HashMap::new();
    for k in tbl.keys() {
        populate_ins(&mut result, &tbl, k);
    }
    result
}

pub fn part1(input: &str) -> Option<String> {
    run_factory(input)
        .into_iter()
        .filter(|(_k, v)| v == &vec![17, 61])
        .map(|x| x.0.rsplit_once(" ").unwrap().1.to_string())
        .next()
}

pub fn part2(input: &str) -> i64 {
    let m = run_factory(input);
    m["output 0"][0] * m["output 1"][0] * m["output 2"][0]
}
