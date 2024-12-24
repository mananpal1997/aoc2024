use std::collections::HashMap;

pub fn solve(_input: &str) -> u64 {
    let mut mappings: HashMap<String, bool> = HashMap::new();
    let mut connections: HashMap<String, (String, String, fn(bool, bool) -> bool)> = HashMap::new();
    let mut indegree: HashMap<String, u64> = HashMap::new();

    _input.lines().for_each(|line| {
        if line.contains(":") {
            let (gate, value) = line.split_once(":").unwrap();
            if value.trim().parse::<u8>().unwrap() == 1 {
                mappings.insert(gate.trim().to_string(), true);
            } else {
                mappings.insert(gate.trim().to_string(), false);
            }
        } else {
            let mut tokens = line.split_whitespace();
            if let (Some(gate1), Some(op), Some(gate2), _, Some(out)) = (tokens.next(), tokens.next(), tokens.next(), tokens.next(), tokens.next()) {
                match op {
                    "XOR" => { connections.insert(out.to_string(), (gate1.to_string(), gate2.to_string(), xor)); },
                    "OR" => { connections.insert(out.to_string(), (gate1.to_string(), gate2.to_string(), or)); },
                    "AND" => { connections.insert(out.to_string(), (gate1.to_string(), gate2.to_string(), and)); },
                    _ => {}
                }
                *indegree.entry(out.to_string()).or_default() += 1;
            }
        }
    });

    connections.keys().filter(|k| k.starts_with("z")).for_each(|out| {
        if mappings.contains_key(out) {
            return;
        }
        calculate_gate_value(out.clone(), &connections, &mut mappings);
    });
    let mut res = 0;
    mappings.iter().filter(|(k, &v)| k.starts_with("z") && v).for_each(|(k, _)| {
        let bit_index = k.replace("z", "").parse::<u64>().unwrap();
        res |= 1 << bit_index;
    });
    res
}

fn calculate_gate_value(gate: String, connections: &HashMap<String, (String, String, fn(bool, bool)->bool)>, mappings: &mut HashMap<String, bool>) -> bool {
    let (gate1, gate2, func) = connections.get(&gate).unwrap();

    let gate1_val = match mappings.contains_key(gate1) {
        true => *mappings.get(gate1).unwrap(),
        false => calculate_gate_value(gate1.clone(), connections, mappings)
    };

    let gate2_val = match mappings.contains_key(gate2) {
        true => *mappings.get(gate2).unwrap(),
        false => calculate_gate_value(gate2.clone(), connections, mappings)
    };

    let gate_val = func(gate1_val, gate2_val);
    mappings.insert(gate, gate_val);
    gate_val
}

fn xor(a: bool, b: bool) -> bool {
    a ^ b
}

fn or(a: bool, b: bool) -> bool {
    a | b
}

fn and(a: bool, b: bool) -> bool {
    a & b
}
