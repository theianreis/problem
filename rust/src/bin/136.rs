// Exercicio 136: Ordene dependencias com o algoritmo topologico de Kahn, detectando ciclos.
use std::collections::VecDeque;
fn ordenar(grafo: &[Vec<usize>]) -> Option<Vec<usize>> {
    let mut graus = vec![0; grafo.len()];
    for &v in grafo.iter().flatten() {
        if v >= grafo.len() {
            return None;
        }
        graus[v] += 1;
    }
    let mut fila: VecDeque<_> = (0..grafo.len()).filter(|&v| graus[v] == 0).collect();
    let mut ordem = Vec::new();
    while let Some(v) = fila.pop_front() {
        ordem.push(v);
        for &u in &grafo[v] {
            graus[u] -= 1;
            if graus[u] == 0 {
                fila.push_back(u);
            }
        }
    }
    if ordem.len() == grafo.len() {
        Some(ordem)
    } else {
        None
    }
}
fn main() {
    let grafo = [vec![1, 2], vec![3], vec![3], vec![]];
    let ordem = ordenar(&grafo).unwrap();
    let mut posicoes = vec![0; grafo.len()];
    for (i, &v) in ordem.iter().enumerate() {
        posicoes[v] = i;
    }
    for (v, vizinhos) in grafo.iter().enumerate() {
        for &u in vizinhos {
            assert!(posicoes[v] < posicoes[u]);
        }
    }
    assert_eq!(ordenar(&[vec![1], vec![0]]), None);
    println!("Ordem das tarefas: {ordem:?}");
}
