use std::collections::HashMap;

pub mod bfs;
pub mod dijkstra;

pub type Node = usize;

// 경로 역 추적 => 도착한 곳만 알뿐 어떤 경로로 왔는지 모를 수도 있기 때문
pub fn reconstruct_path(parent: &HashMap<Node, Node>, start: Node, goal: Node) -> Vec<Node> {
    let mut path = vec![goal];
    let mut current = goal;

    while current != start {
        current = *parent.get(&current).unwrap();
        path.push(current); // 이전 위치의 노드 => 역으로 하나씩 push
    }

    path.reverse();
    path
}
