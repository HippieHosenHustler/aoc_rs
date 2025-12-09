use std::collections::HashSet;

use shared::lines_to_vec;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Point { x, y, z }
    }

    fn from_string(input: String) -> Self {
        input
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
            .as_slice()
            .try_into()
            .map(|arr: [i32; 3]| Point::new(arr[0], arr[1], arr[2]))
            .unwrap()
    }

    fn distance(&self, other: &Point) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

struct Circuit<'a> {
    points: HashSet<&'a Point>,
}

impl Circuit<'_> {
    fn new() -> Self {
        Circuit {
            points: HashSet::new(),
        }
    }

    fn size(&self) -> usize {
        self.points.len()
    }
}

struct Distance<'a> {
    from: &'a Point,
    to: &'a Point,
    distance: f64,
}

pub fn solve(input: &str) -> (String, String) {
    let lines = lines_to_vec(input);

    let points: Vec<Point> = lines.into_iter().map(Point::from_string).collect();

    let part_1 = solve_part_1(points);

    (part_1, String::new())
}

fn solve_part_1(points: Vec<Point>) -> String {
    let mut circuits: Vec<Circuit> = Vec::new();

    find_all_distances_sorted(&points)
        .into_iter()
        .take(1000)
        .for_each(|distance| {
            let mut circuit_from_idx = Option::None;
            let mut circuit_to_idx = Option::None;

            for (idx, circuit) in circuits.iter().enumerate() {
                if circuit.points.contains(distance.from) {
                    circuit_from_idx = Some(idx);
                }
                if circuit.points.contains(distance.to) {
                    circuit_to_idx = Some(idx);
                }
            }

            match (circuit_from_idx, circuit_to_idx) {
                (Some(c_from), Some(c_to)) => {
                    if c_from != c_to {
                        let points_to_add: Vec<&Point> =
                            circuits[c_to].points.iter().cloned().collect();
                        for point in points_to_add {
                            circuits[c_from].points.insert(point);
                        }
                        circuits.remove(c_to);
                    }
                }
                (Some(c_from), None) => {
                    circuits[c_from].points.insert(distance.to);
                }
                (None, Some(c_to)) => {
                    circuits[c_to].points.insert(distance.from);
                }
                (None, None) => {
                    let mut new_circuit = Circuit::new();
                    new_circuit.points.insert(distance.from);
                    new_circuit.points.insert(distance.to);
                    circuits.push(new_circuit);
                }
            }
        });

    circuits.sort_by(|a, b| b.size().cmp(&a.size()));

    circuits
        .iter()
        .take(3)
        .map(|c| c.size())
        .product::<usize>()
        .to_string()
}

fn find_all_distances(points: &[Point]) -> Vec<Distance> {
    let mut distances = Vec::new();

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let distance = points[i].distance(&points[j]);
            distances.push(Distance {
                from: &points[i],
                to: &points[j],
                distance,
            });
        }
    }

    distances
}

fn find_all_distances_sorted(points: &[Point]) -> Vec<Distance> {
    let mut distances = find_all_distances(points);
    distances.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    distances
}
