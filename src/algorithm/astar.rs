use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Node {
    position: (i32, i32),
    f: i32, // 총 점수(g(비용) + h(휴리스틱 추정 값))
    g: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.f.cmp(&self.f))
    }
}

fn heuristic(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn in_bounds(pos: (i32, i32), width: i32, height: i32) -> bool {
    pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height
}

fn astar(grid: &Vec<Vec<i32>>, start: (i32, i32), goal: (i32, i32)) -> Option<Vec<(i32, i32)>> {
    let mut heap = BinaryHeap::new();
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut g_score: HashMap<(i32, i32), i32> = HashMap::new();

    heap.push(Node {
        position: start,
        f: heuristic(start, goal),
        g: 0,
    });

    g_score.insert(start, 0);

    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    while let Some(current) = heap.pop() {
        if current.position == goal {
            let mut path = vec![goal];
            let mut cur = goal;

            while let Some(prev) = came_from.get(&cur) {
                path.push(*prev);
                cur = *prev;
            }

            path.reverse();
            return Some(path);
        }

        for dir in &directions {
            let neighbor = (current.position.0 + dir.0, current.position.1 + dir.1);

            if !in_bounds(neighbor, grid[0].len() as i32, grid.len() as i32) {
                continue;
            }

            if grid[neighbor.1 as usize][neighbor.0 as usize] == 1 {
                continue;
            }

            let tentative_g = current.g + 1;
            let best_g = g_score.get(&neighbor).unwrap_or(&i32::MAX);

            // 지금까지 알던 경로보다 더 짧은 경우 또는 아직 가본 적 없는 경우 로직 실행
            if tentative_g < *best_g {
                came_from.insert(neighbor, current.position);
                g_score.insert(neighbor, tentative_g);

                let f = tentative_g + heuristic(neighbor, goal);

                heap.push(Node {
                    position: neighbor,
                    f,
                    g: tentative_g,
                });
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_astar() {
        let grid = vec![
            vec![0, 0, 0, 1, 0],
            vec![1, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];

        let start = (0, 0);
        let goal = (4, 2);

        match astar(&grid, start, goal) {
            Some(path) => {
                println!("경로:");
                for p in path {
                    println!("{:?}", p);
                }
            }
            None => println!("경로 없음"),
        }
    }
}
