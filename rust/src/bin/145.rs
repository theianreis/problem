// Exercicio 145: Reconstrua o menor caminho em numero de arestas usando BFS e predecessores.
use std::collections::VecDeque;
fn menor_caminho(grafo: &[Vec<usize>], origem: usize, destino: usize) -> Option<Vec<usize>> {
    let n = grafo.len();
    if origem >= n || destino >= n || grafo.iter().flatten().any(|&v| v >= n) {
        return None;
    }
    let mut pais = vec![None; n];
    pais[origem] = Some(origem);
    let mut fila = VecDeque::from([origem]);
    while let Some(v) = fila.pop_front() {
        if v == destino {
            break;
        }
        for &u in &grafo[v] {
            if pais[u].is_none() {
                pais[u] = Some(v);
                fila.push_back(u);
            }
        }
    }
    pais[destino]?;
    let mut caminho = vec![destino];
    while *caminho.last()? != origem {
        caminho.push(pais[*caminho.last()?]?);
    }
    caminho.reverse();
    Some(caminho)
}
fn main() {
    let grafo = [vec![1, 2], vec![3], vec![3], vec![4], vec![], vec![]];
    assert_eq!(menor_caminho(&grafo, 0, 4), Some(vec![0, 1, 3, 4]));
    assert_eq!(menor_caminho(&grafo, 0, 5), None);
    assert_eq!(menor_caminho(&grafo, 2, 2), Some(vec![2]));
    println!("Menor caminho: {:?}", menor_caminho(&grafo, 0, 4));
}
