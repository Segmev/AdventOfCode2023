use itertools::Itertools;
use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Clone)]
struct Node {
    name: String,
    children: HashSet<String>,
}

fn get_all_edges(nodes: &BTreeMap<String, Node>) -> HashSet<(String, String)> {
    let mut edges: HashSet<(String, String)> = HashSet::new();
    for node in nodes.values() {
        for child in node.children.iter() {
            let mut edge_names = vec![node.name.clone(), child.clone()];
            edge_names.sort();
            edges.insert((edge_names[0].clone(), edge_names[1].clone()));
        }
    }
    edges
}

pub fn start(file_content: &str) {
    let mut nodes = BTreeMap::new();

    for line in file_content.lines() {
        let (name, children) = line.split_once(": ").unwrap();
        let children: Vec<&str> = children.split_whitespace().collect();

        let node = Node {
            name: name.to_string(),
            children: HashSet::new(),
        };
        nodes
            .entry(node.name.clone())
            .or_insert(node)
            .children
            .extend(
                children
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>(),
            );

        for child in children {
            nodes
                .entry(child.to_string())
                .or_insert(Node {
                    name: child.to_string(),
                    children: HashSet::new(),
                })
                .children
                .insert(name.to_string());
        }
    }

    let edges_list = get_all_edges(&nodes);
    let mut edges_cost: Vec<((String, String), i32)> = vec![];

    for edges in edges_list.iter() {
        let (node_start, node_end) = edges;
        nodes.get_mut(node_start).unwrap().children.remove(node_end);

        let mut investigated: BTreeMap<String, i32> = BTreeMap::new();
        let mut to_investigate = vec![(node_start.to_string(), 0)];
        let mut longest = 0;
        while to_investigate.len() > 0 {
            let n = to_investigate.remove(0);
            if investigated.contains_key(&n.0) {
                continue;
            }
            if n.0 == node_end.to_string() {
                longest = n.1;
                break;
            }
            investigated.insert(n.0.clone(), n.1);

            to_investigate.extend(
                nodes
                    .get(&n.0)
                    .unwrap()
                    .children
                    .iter()
                    .map(|c| (c.to_string(), n.1 + 1)),
            );
        }
        edges_cost.push((edges.clone(), longest));
        nodes
            .get_mut(node_start)
            .unwrap()
            .children
            .insert(node_end.clone());
    }
    edges_cost.sort_by(|a, b| b.1.cmp(&a.1));
    edges_cost = edges_cost.into_iter().take(15).collect();

    for ids in (0..edges_cost.len()).combinations(3) {
        for e in [
            edges_cost[ids[0]].clone(),
            edges_cost[ids[1]].clone(),
            edges_cost[ids[2]].clone(),
        ] {
            nodes.get_mut(&e.0 .0).unwrap().children.remove(&e.0 .1);
            nodes.get_mut(&e.0 .1).unwrap().children.remove(&e.0 .0);
        }
        let first_group = get_group_nodes(edges_cost[ids[0]].0 .0.as_str(), &nodes);
        let second_group = get_group_nodes(edges_cost[ids[0]].0 .1.as_str(), &nodes);

        if nodes.len() != first_group.len() + second_group.len() {
            for e in [
                edges_cost[ids[0]].clone(),
                edges_cost[ids[1]].clone(),
                edges_cost[ids[2]].clone(),
            ] {
                nodes
                    .get_mut(&e.0 .0.clone())
                    .unwrap()
                    .children
                    .insert(e.0 .1.clone());
                nodes.get_mut(&e.0 .1).unwrap().children.insert(e.0 .0);
            }

            continue;
        };
        println!(
            "{:?} {:?} {:?}",
            edges_cost[ids[0]], edges_cost[ids[1]], edges_cost[ids[2]]
        );
        println!("{}", first_group.len() * second_group.len());
        return;
    }
}

fn get_group_nodes(node: &str, nodes: &BTreeMap<String, Node>) -> HashSet<String> {
    let mut investigated: HashSet<String> = HashSet::new();
    let mut to_investigate = vec![(node.to_string(), 0)];
    while to_investigate.len() > 0 {
        let n = to_investigate.remove(0);
        if investigated.contains(&n.0) {
            continue;
        }
        investigated.insert(n.0.clone());

        to_investigate.extend(
            nodes
                .get(&n.0)
                .unwrap()
                .children
                .iter()
                .map(|c| (c.to_string(), n.1 + 1)),
        );
    }

    investigated
}
