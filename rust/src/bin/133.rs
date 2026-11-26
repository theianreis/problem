// Exercicio 133: Percorra um grafo em profundidade usando uma pilha explicita.
fn dfs(grafo: &[Vec<usize>], origem: usize) -> Option<Vec<usize>> {
    if origem >= grafo.len() || grafo.iter().flatten().any(|&v| v >= grafo.len()) {
        return None;
    }
    let mut pilha = vec![origem];
    let mut visitados = vec![false; grafo.len()];
    let mut ordem = Vec::new();
    while let Some(v) = pilha.pop() {
        if visitados[v] {
            continue;
        }
        visitados[v] = true;
        ordem.push(v);
        pilha.extend(grafo[v].iter().rev().copied().filter(|&u| !visitados[u]));
    }
    Some(ordem)
}
fn main() {
    let grafo = vec![vec![1, 2], vec![3], vec![3], vec![0]];
    assert_eq!(dfs(&grafo, 0), Some(vec![0, 1, 3, 2]));
    println!("DFS: {:?}", dfs(&grafo, 0));
}
