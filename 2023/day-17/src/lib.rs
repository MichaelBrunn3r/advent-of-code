use itertools::Itertools;

// 1000 too low
// 1100 too high
// 1104 too high
pub fn part_1(input: &mut str) -> usize {
    let map = parse_map(input);
    let size = map.len();

    let start = Node::new(0usize, 0usize);
    let goal = Node::new(size - 1, size - 1);

    let mut paths = vec![Path::new(start)];

    let mut iter = 0;
    let mut res = None;
    while !paths.is_empty() {
        iter += 1;
        let path = paths.remove(0);
        let last = *path.last();
        if last == goal {
            res = Some(path);
            break;
        }

        // Expand
        let mut expanded_paths = Vec::new();
        for i in (1..=3).rev() {
            if path.last_dir != Direction::Left && path.last_dir != Direction::Right {
                // Expand right
                if last.col + i < size {
                    let mut p = path.clone();
                    let right = last.right(i);

                    // Check for loop
                    if !p.contains(&right) {
                        let cost = (1..=i).map(|j| map[last.row][last.col + j] as usize).sum();
                        p.add(
                            right,
                            cost,
                            right.manhattan_distance(goal),
                            Direction::Right,
                        );
                        expanded_paths.push(p);
                    }
                }

                // Expand left
                if last.col >= i {
                    let mut p = path.clone();
                    let left = last.left(i);

                    // Check for loop
                    if !p.contains(&left) {
                        let cost = (1..=i).map(|j| map[last.row][last.col - j] as usize).sum();
                        p.add(
                            left,
                            cost,
                            left.manhattan_distance(goal) + 1,
                            Direction::Left,
                        );
                        expanded_paths.push(p);
                    }
                }
            }

            if path.last_dir != Direction::Up && path.last_dir != Direction::Down {
                // Expand down
                if last.row + i < size {
                    let mut p = path.clone();
                    let down = last.down(i);

                    // Check for loop
                    if !p.contains(&down) {
                        let cost = (1..=i).map(|j| map[last.row + j][last.col] as usize).sum();
                        p.add(down, cost, down.manhattan_distance(goal), Direction::Down);
                        expanded_paths.push(p);
                    }
                }

                // Expand up
                if last.row >= i {
                    let mut p = path.clone();
                    let up = last.up(i);

                    // Check for loop
                    if !p.contains(&up) {
                        let cost = (1..=i).map(|j| map[last.row - j][last.col] as usize).sum();
                        p.add(up, cost, up.manhattan_distance(goal) + 1, Direction::Up);
                        expanded_paths.push(p);
                    }
                }
            }
        }

        // If path that ends in the same node exists, keep the one with the lowest cost
        expanded_paths.retain(|p| {
            let last = *p.last();
            if let Some(other_pos) = paths.iter().position(|p| *p.last() == last) {
                let other = paths.get(other_pos).unwrap();
                return p.cost < other.cost;
            };
            true
        });

        paths.extend(expanded_paths);

        paths.sort_by_key(|p| p.cost);

        if iter % 10_000 == 0 {
            println!("Iterations: {}", iter);
            print_path_on_map(&map, &paths[0]);
        }
    }

    let res = res.unwrap();
    print_path_on_map(&map, &res);
    println!("Iterations: {}", iter);
    println!("Path: {:?}", res);

    res.cost
}

pub fn part_2(input: &str) -> usize {
    0
}

fn parse_map(input: &str) -> Vec<Vec<u8>> {
    let size = input.find('\n').unwrap();

    // Convert numbers to integers
    input
        .as_bytes()
        .chunks_exact(size + 1)
        .map(|line| line[0..size].iter().map(|b| b - b'0').collect_vec())
        .collect_vec()
}

#[derive(Debug, Clone)]
struct Path {
    nodes: Vec<Node>,
    cost: usize,
    estimated_rest_cost: usize,
    last_dir: Direction,
}

impl Path {
    fn new(start: Node) -> Self {
        Self {
            nodes: vec![start],
            cost: 0,
            last_dir: Direction::None,
            estimated_rest_cost: 0,
        }
    }

    fn last(&self) -> &Node {
        self.nodes.last().unwrap()
    }

    fn contains(&self, node: &Node) -> bool {
        self.nodes.contains(node)
    }

    fn add(&mut self, node: Node, cost: usize, estimated_rest_cost: usize, dir: Direction) {
        self.nodes.push(node);
        self.cost += cost;
        self.estimated_rest_cost = estimated_rest_cost;
        self.last_dir = dir;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Node {
    row: usize,
    col: usize,
}

impl Node {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn manhattan_distance(&self, other: Self) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }

    fn left(&self, n: usize) -> Self {
        Self {
            row: self.row,
            col: self.col - n,
        }
    }

    fn right(&self, n: usize) -> Self {
        Self {
            row: self.row,
            col: self.col + n,
        }
    }

    fn up(&self, n: usize) -> Self {
        Self {
            row: self.row - n,
            col: self.col,
        }
    }

    fn down(&self, n: usize) -> Self {
        Self {
            row: self.row + n,
            col: self.col,
        }
    }
}

fn print_path_on_map(map: &[Vec<u8>], path: &Path) {
    let mut pretty = vec![vec![b'.'; map.len()]; map.len()];

    path.nodes.iter().tuple_windows().for_each(|(prev, curr)| {
        if prev.row < curr.row {
            for i in prev.row + 1..=curr.row {
                pretty[i][prev.col] = b'v';
            }
        } else if prev.row > curr.row {
            for i in curr.row..prev.row {
                pretty[i][prev.col] = b'^';
            }
        } else if prev.col < curr.col {
            for i in prev.col + 1..=curr.col {
                pretty[prev.row][i] = b'>';
            }
        } else if prev.col > curr.col {
            for i in curr.col..prev.col {
                pretty[prev.row][i] = b'<';
            }
        }
    });
    println!(
        "{}",
        pretty
            .iter()
            .map(|line| String::from_utf8_lossy(line))
            .join("\n")
    );
}
