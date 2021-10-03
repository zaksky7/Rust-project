use std::collections::HashMap;

fn parse_orbits<'a>(input: &'a str) -> impl Iterator<Item = (&'a str, &'a str)> {
    input.lines().map(|line| {
        let pts: Vec<&str> = line.split(')').collect();
        (pts[0], pts[1])
    })
}

pub fn part1(input: &str) -> usize {
    let mut t = HashMap::new();
    for (k, v) in parse_orbits(input) {
        let e = t.entry(k.to_string()).or_insert(vec![]);
        (*e).push(v);
    }
    let mut depth = 0;
    let mut keys = vec!["COM"];
    let mut result = 0;
    while !keys.is_empty() {
        result += depth * keys.len();
        keys = keys
            .into_iter()
            .flat_map(|x| t.get(x).unwrap_or(&vec![]).clone())
            .collect();
        depth += 1;
    }
    result
}

fn path_from_com<'a>(t: &'a HashMap<&str, &str>, key: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut k = key;
    while t.contains_key(k) {
        k = &t[k];
        result.push(k);
    }
    result.reverse();
    result
}

pub fn part2(input: &str) -> usize {
    let t = parse_orbits(input)
        .map(|(k, v)| (v, k))
        .collect::<HashMap<_, _>>();
    let xs = path_from_com(&t, "YOU");
    let ys = path_from_com(&t, "SAN");
    for (i, (x, y)) in xs.iter().zip(ys.iter()).enumerate() {
        if x != y {
            return xs.len() + ys.len() - 2 * i;
        }
    }
    0
}
