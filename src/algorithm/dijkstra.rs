use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    u32,
};

use crate::algorithm::{Node, reconstruct_path};

type WeightedGraph = HashMap<Node, Vec<(Node, u32)>>;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32, // 시작점부터 해당 노드까지의 거리/비용
    node: Node,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // 비교하려는 노드의 비용을 현재 노드의 비용과 비교한다.
        // self.cost.cmp(&other.cost)로 하지 않는 이유는 역순으로 나타내기 위함
        // 다익스트라는 비용이 작은 게 먼저 나와야 함
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node)) // 비용이 같을 때 노드 번호로 정렬 => 비용과 달리 노드 번호는 우선순위가 아니기 때문에 오름차순 정렬
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra(graph: &WeightedGraph, start: Node, goal: Node) -> Option<(Vec<Node>, u32)> {
    let mut distances: HashMap<Node, u32> = HashMap::new(); // 노드까지의 최단 거리
    let mut parent: HashMap<Node, Node> = HashMap::new(); // 경로 추적
    let mut heap = BinaryHeap::new();

    distances.insert(start, 0);
    heap.push(State {
        cost: 0,
        node: start,
    });

    while let Some(State {
        cost,
        node: current,
    }) = heap.pop()
    {
        if current == goal {
            let path = reconstruct_path(&parent, start, goal);
            return Some((path, cost));
        }

        if cost > *distances.get(&current).unwrap_or(&u32::MAX) {
            continue; // 기존의 최단 거리과 비교
        }

        if let Some(neighbors) = graph.get(&current) {
            for &(neighbor, weight) in neighbors {
                let next_cost = cost + weight;

                // 다른 곳에서 저장된 비용이 있을 수도 있기에 비교 후 더 작은 비용을 저장
                if next_cost < *distances.get(&neighbor).unwrap_or(&u32::MAX) {
                    distances.insert(neighbor, next_cost);
                    parent.insert(neighbor, current); // 경로 추적을 위해 삽입
                    heap.push(State {
                        cost: next_cost,
                        node: neighbor,
                    });
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_simple() {
        let mut graph: WeightedGraph = HashMap::new();
        graph.insert(0, vec![(1, 5), (2, 1)]);
        graph.insert(1, vec![(3, 2)]);
        graph.insert(2, vec![(3, 8)]);
        graph.insert(3, vec![]);

        if let Some((path, cost)) = dijkstra(&graph, 0, 3) {
            assert_eq!(path[0], 0);
            assert_eq!(path[1], 1);
            assert_eq!(path[2], 3);
            assert_eq!(cost, 7);
        }
    }
}
