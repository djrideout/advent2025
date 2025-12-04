use std::{collections::{HashMap, HashSet, VecDeque}, hash::Hash};
use regex::Regex;

pub struct Input {
    pub input: String,
    pub line_len: i32
}

pub fn generate_input(in_lines: &str) -> Input {
    let new_line_regex = Regex::new(r"\n").unwrap();
    Input {
        input: new_line_regex.replace_all(in_lines, "").to_string(),
        line_len: new_line_regex.find(in_lines).unwrap().start() as i32
    }
}

pub struct Grid {
    data: String,
    line_len: i32
}

impl Grid {
    pub fn new(data: String, line_len: i32) -> Grid {
        Grid {
            data,
            line_len
        }
    }

    pub fn is_inbounds(&self, pos: i32, curr_pos: i32, line_offset: i32) -> bool {
        if pos < 0 || pos >= self.data.len() as i32 || curr_pos < 0 || curr_pos >= self.data.len() as i32 {
            return false;
        }
        let curr_row = curr_pos / self.line_len;
        let other_row_start = (curr_row + line_offset) * self.line_len;
        let other_row_end = other_row_start + self.line_len - 1;
        if pos >= other_row_start && pos <= other_row_end {
            return true;
        }
        false
    }

    pub fn get_char(&self, pos: i32, direction: usize, distance: i32) -> (char, i32) {
        let (char_offset, line_offset) = match direction {
            0 => (-self.line_len - 1, -1), // Up-left
            1 => (-self.line_len, -1),     // Up
            2 => (-self.line_len + 1, -1), // Up-right
            3 => (-1, 0),                  // Left
            4 => (1, 0),                   // Right
            5 => (self.line_len - 1, 1),   // Down-left
            6 => (self.line_len, 1),       // Down
            7 => (self.line_len + 1, 1),   // Down-right
            _ => (0, 0)                    // Oh no
        };
        let char_pos = pos + char_offset * distance;
        if self.is_inbounds(char_pos, pos + char_offset * (distance - 1), line_offset) {
            return (self.data.chars().nth(char_pos as usize).unwrap(), char_pos);
        }
        ('!', char_pos)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Edge {
    dest: i32,
    distance: i32,
    positions: Vec<i32>,
    cost: i32,
    direction: usize
}

fn is_position_node(pos: i32, end_pos: i32, direction: usize, grid: &Grid) -> bool {
    if pos == end_pos {
        return true;
    }
    for dir in get_perpendicular_directions(direction) {
        let (next_char, _) = grid.get_char(pos, dir, 1);
        if next_char != '#' && next_char != '!' {
            return true;
        }
    }
    false
}

pub fn get_opposite_direction(direction: usize) -> usize {
    7 - direction
}

pub fn get_perpendicular_directions(direction: usize) -> [usize; 2] {
    match direction {
        0 => [2, 5],
        1 => [3, 4],
        2 => [0, 7],
        3 => [1, 6],
        4 => [1, 6],
        5 => [0, 7],
        6 => [3, 4],
        7 => [2, 5],
        _ => [0, 0]
    }
}

fn get_neighbors(pos: i32, end_pos: i32, direction: usize, opposite_cost: i32, perpendicular_cost: i32, grid: &Grid) -> Vec<Edge> {
    let mut neighbors: Vec<Edge> = vec![];
    for next_direction in [1, 3, 4, 6] {
        let turn_cost = if direction == next_direction {
            0
        } else if get_opposite_direction(direction) == next_direction {
            opposite_cost
        } else {
            perpendicular_cost
        };
        let mut positions = vec![pos];
        loop {
            let (next_char, next_pos) = grid.get_char(*positions.last().unwrap(), next_direction, 1);
            if next_char == '#' || next_char == '!' {
                break;
            }
            let distance = positions.len() as i32;
            positions.push(next_pos);
            if is_position_node(next_pos, end_pos, next_direction, grid) {
                neighbors.push(Edge {
                    dest: next_pos,
                    distance,
                    positions: positions.clone(),
                    cost: turn_cost + distance,
                    direction: next_direction
                });
            }
        }
    }
    neighbors
}

pub struct DijkstraResult {
    pub positions: HashSet<i32>,
    pub min_distances: HashMap<i32, i32>
}

pub fn dijkstra(map: &String, line_len: i32, start_pos: i32, end_pos: i32, opposite_cost: i32, perpendicular_cost: i32) -> DijkstraResult {
    let grid = Grid::new(map.clone(), line_len);
    let mut min_distances: HashMap::<i32, i32> = HashMap::new();
    let mut node_queue: VecDeque::<(i32, usize)> = VecDeque::new();
    let mut visited_positions: Vec<i32> = vec![];
    let mut prev_edges: HashMap::<i32, Vec<Edge>> = HashMap::new();
    min_distances.insert(start_pos, 0);
    node_queue.push_back((start_pos, 4));
    while node_queue.len() > 0 {
        let (pos, direction) = node_queue.pop_front().unwrap();
        let curr_distance = *min_distances.get(&pos).unwrap();
        for neighbor in get_neighbors(pos, end_pos, direction, opposite_cost, perpendicular_cost, &grid) {
            if visited_positions.contains(&neighbor.dest) {
                continue;
            }
            let min_distance = min_distances.get(&neighbor.dest);
            let new_distance = curr_distance + neighbor.cost;
            if !min_distance.is_some() || new_distance <= *min_distance.unwrap() {
                min_distances.insert(neighbor.dest, new_distance);
                if let Some(destination_edges) = prev_edges.get_mut(&neighbor.dest) {
                    destination_edges.push(neighbor.clone());
                } else {
                    prev_edges.insert(neighbor.dest, vec![neighbor.clone()]);
                }
                if let Some(index) = node_queue.iter().position(|(p, _)| *p == neighbor.dest) {
                    node_queue.remove(index);
                }
                node_queue.push_back((neighbor.dest, neighbor.direction));
            }
            node_queue.make_contiguous().sort_by(|(a, _), (b, _)| {
                let a_dist = min_distances.get(&a).unwrap();
                let b_dist = min_distances.get(&b).unwrap();
                return a_dist.cmp(b_dist);
            });
        }
        visited_positions.push(pos);
    }
    let mut positions: HashSet<i32> = HashSet::new();
    let end_prev_edges = prev_edges.get(&end_pos);
    if end_prev_edges.is_some() {
        let mut edges = HashSet::from_iter(end_prev_edges.unwrap());
        while edges.len() > 0 {
            let mut new_edges: HashSet<&Edge> = HashSet::new();
            for edge in edges {
                edge.positions.iter().for_each(|p| {
                    positions.insert(*p);
                });
                let (_, source_pos) = grid.get_char(edge.dest, get_opposite_direction(edge.direction), edge.distance);
                if let Some(source_edges) = prev_edges.get(&source_pos) {
                    source_edges.iter().for_each(|edge| {
                        new_edges.insert(edge);
                    });
                }
            }
            edges = new_edges;
        }
    }
    DijkstraResult {
        positions,
        min_distances
    }
}
