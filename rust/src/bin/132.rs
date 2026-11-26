// Exercicio 132: Percorra um grafo em largura usando uma fila e marcacao de visitados.
use std::collections::VecDeque;
fn bfs(grafo: &[Vec<usize>], origem: usize) -> Option<Vec<usize>> {
    if origem >= grafo.len() || grafo.iter().flatten().any(|&v| v >= grafo.len()) {
        return None;
    }
    let mut fila = VecDeque::from([origem]);
    let mut visitados = vec![false; grafo.len()];
    visitados[origem] = true;
    let mut ordem = Vec::new();
    while let Some(v) = fila.pop_front() {
        ordem.push(v);
        for &vizinho in &grafo[v] {
            if !visitados[vizinho] {
                visitados[vizinho] = true;
                fila.push_back(vizinho);
            }
        }
    }
    Some(ordem)
}
fn main() {
    let grafo = vec![vec![1, 2], vec![3], vec![3], vec![], vec![]];
    assert_eq!(bfs(&grafo, 0), Some(vec![0, 1, 2, 3]));
    assert_eq!(bfs(&grafo, 5), None);
    println!("BFS: {:?}", bfs(&grafo, 0));
}
