use std::collections::{HashMap, HashSet, VecDeque};

use crate::algorithm::{Node, reconstruct_path};

pub type Graph = HashMap<Node, Vec<Node>>;

pub fn bfs(graph: &Graph, start: Node, goal: Node) -> Option<Vec<Node>> {
    let mut queue = VecDeque::new(); // 큐
    let mut visited = HashSet::new(); // 방문 체크
    let mut parent = HashMap::new(); // 경로 추적

    queue.push_back(start);
    visited.insert(start);

    // 노드 꺼내기
    while let Some(current) = queue.pop_front() {
        // 노드가 목표에 도달한 경우 경로 역추적해서 반환
        if current == goal {
            return Some(reconstruct_path(&parent, start, goal));
        }

        // 노드가 아직 목표에 도달하지 않은 경우 현재 노드와 연결된 근처 노드 조회
        if let Some(neighbors) = graph.get(&current) {
            for &neighbor in neighbors {
                // 근처 노드에 방문한 적 없으면 방문 기록 남기고 노드로 이동
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    parent.insert(neighbor, current);
                    queue.push_back(neighbor);
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
    fn test_bfs_simple() {
        // 특정 노드에 연결된 엣지들
        let mut graph: Graph = HashMap::new();

        // 0번 노드에 1과 2로 이어지는 노드 존재
        graph.insert(0, vec![1, 2]);

        // 1번 노드에 3으로 이어지는 노드 존재
        graph.insert(1, vec![3]);

        // 2번 노드에 3으로 이어지는 노드 존재
        graph.insert(2, vec![3]);

        // 3번 노드에 아무런 엣지 존재 X
        graph.insert(3, vec![]);

        if let Some(path) = bfs(&graph, 0, 3) {
            assert_eq!(path[0], 0);
            assert_eq!(path[1], 1);
            assert_eq!(path[2], 3);
        }
    }
}
