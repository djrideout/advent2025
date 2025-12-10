use std::cmp::{min, max};

type Input = Vec<(i64, i64)>;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Input {
    input.lines().map(|l| {
        let mut nums = l.split(',');
        (
            nums.nth(0).unwrap().parse::<i64>().unwrap(),
            nums.nth(0).unwrap().parse::<i64>().unwrap()
        )
    }).collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &Input) -> i64 {
    let mut max_area = 0;
    input
        .iter()
        .enumerate()
        .for_each(|(i, a)| {
            input.iter().skip(i + 1).for_each(|b| {
                let area = ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1);
                if area > max_area {
                    max_area = area;
                }
            });
        });
    max_area
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Input) -> i64 {
    let mut edges = vec![];
    input
        .iter()
        .enumerate()
        .for_each(|(i, a)| {
            input.iter().skip(i + 1).for_each(|b| {
                if a.0 == b.0 || a.1 == b.1 {
                    edges.push((a, b));
                }
            });
        });
    let mut max_area = 0;
    input
        .iter()
        .enumerate()
        .for_each(|(i, a)| {
            input.iter().skip(i + 1).for_each(|b| {
                // check that all 4 points of the rectangle are in the polygon or on the boundary.
                // we don't need to check the two points from the input,
                // as those two are trivially on the polygon boundary
                let virtual_points = vec![
                    (a.0, b.1),
                    (b.0, a.1)
                ];
                let mut intersections = 0;
                'point_loop: for point in &virtual_points {
                    let mut line_intersections = 0;
                    for edge in edges.clone() {
                        if edge.0.0 == edge.1.0 {
                            // vertical edge
                            if point.0 == edge.0.0 && point.1 >= min(edge.0.1, edge.1.1) && point.1 <= max(edge.0.1, edge.1.1) {
                                // on boundary
                                intersections += 1;
                                continue 'point_loop;
                            }
                            if point.0 > edge.0.0 && point.1 >= min(edge.0.1, edge.1.1) && point.1 < max(edge.0.1, edge.1.1) {
                                // not on boundary, but left-ray intersecting middle or low end (exclude high end)
                                line_intersections += 1;
                            }
                        } else {
                            // horizontal edge
                            if point.1 == edge.0.1 && point.0 >= min(edge.0.0, edge.1.0) && point.0 <= max(edge.0.0, edge.1.0) {
                                // on boundary
                                intersections += 1;
                                continue 'point_loop;
                            }
                        }
                    }
                    if line_intersections % 2 == 1 {
                        intersections += 1;
                    }
                }
                if intersections != virtual_points.len() {
                    // not all 4 rect points are in polygon
                    return;
                }
                // now need to check if any of the rectangle edges intersect with polygon edges
                // if they do, that means the rectangle is outside of the polygon at some point
                // due to a concave dip in the polygon
                let rect_edges = vec![
                    (*a, (a.0, b.1)),
                    (*a, (b.0, a.1)),
                    (*b, (a.0, b.1)),
                    (*b, (b.0, a.1))
                ];
                for edge in edges.clone() {
                    for rect_edge in rect_edges.clone() {
                        if rect_edge.0.0 == rect_edge.1.0 && edge.0.1 == edge.1.1 {
                            // vertical rect edge, horizontal polygon edge
                            if
                                rect_edge.0.0 > min(edge.0.0, edge.1.0) &&
                                rect_edge.0.0 < max(edge.0.0, edge.1.0) &&
                                edge.0.1 > min(rect_edge.0.1, rect_edge.1.1) &&
                                edge.0.1 < max(rect_edge.0.1, rect_edge.1.1)
                            {
                                return;
                            }
                        } else if rect_edge.0.1 == rect_edge.1.1 && edge.0.0 == edge.1.0 {
                            // horizontal rect edge, vertical polygon edge
                            if
                                rect_edge.0.1 > min(edge.0.1, edge.1.1) &&
                                rect_edge.0.1 < max(edge.0.1, edge.1.1) &&
                                edge.0.0 > min(rect_edge.0.0, rect_edge.1.0) &&
                                edge.0.0 < max(rect_edge.0.0, rect_edge.1.0)
                            {
                                return;
                            }
                        }
                    }
                }
                let area = ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1);
                if area > max_area {
                    max_area = area;
                }
            });
        });
    max_area
}
