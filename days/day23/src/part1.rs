use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve(_input: &str) -> usize {
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    _input.lines().for_each(|line| {
        let (c1, c2) = line.split_once("-").unwrap();
        graph.entry(c1).or_default().insert(c2);
        graph.entry(c2).or_default().insert(c1);
    });

    let mut cliques: Vec<Vec<&str>> = Vec::new();
    let mut index: HashMap<&str, usize> = HashMap::new();
    let mut nbrs: HashMap<&str, HashSet<&str>> = HashMap::new();

    for &u in graph.keys() {
        index.insert(u, index.len());
        for &adj in graph.get(u).unwrap_or(&HashSet::new()) {
            if index.contains_key(adj) {
                continue;
            }
            nbrs.entry(u).or_default().insert(adj);
        }
    }

    let mut queue = VecDeque::new();
    for &u in graph.keys() {
        let mut sorted_nbrs: Vec<&str> = nbrs.get(u).unwrap_or(&HashSet::new()).iter().map(|&v| v).collect();
        sorted_nbrs.sort_by_key(|&k| index.get(k).unwrap());
        queue.push_back((Vec::from([u]), sorted_nbrs));
    }

    while let Some((base, cnbrs)) = queue.pop_front() {
        if base.len() > 3 {
            continue;
        }
        if base.len() == 3 {
            cliques.push(base);
            continue;
        }
        for (i, &u) in cnbrs.iter().enumerate() {
            let mut new_base = base.clone();
            new_base.push(u);

            let filter: HashSet<&&str> = cnbrs.iter().skip(i+1).collect();
            let mut new_cnbrs: Vec<&str> = nbrs.get(u).unwrap_or(&HashSet::new()).iter().filter(|v| filter.contains(v)).map(|&v| v).collect();
            new_cnbrs.sort_by_key(|&k| index.get(k).unwrap());
            queue.push_back((new_base, new_cnbrs));
        }
    }
    cliques.iter().filter(|&clique| clique.iter().any(|c| c.starts_with("t"))).count()
}