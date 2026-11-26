// Exercicio 137: Calcule caminhos minimos com Dijkstra e pesos nao negativos.
use std::{cmp::Reverse, collections::BinaryHeap};
fn dijkstra(grafo: &[Vec<(usize, u64)>], origem: usize) -> Result<Vec<Option<u64>>, &'static str> {
    if origem >= grafo.len() || grafo.iter().flatten().any(|&(v, _)| v >= grafo.len()) {
        return Err("vertice invalido");
    }
    let mut distancias = vec![None; grafo.len()];
    distancias[origem] = Some(0);
    let mut heap = BinaryHeap::from([Reverse((0_u64, origem))]);
    while let Some(Reverse((distancia, v))) = heap.pop() {
        if distancias[v] != Some(distancia) {
            continue;
        }
        for &(u, peso) in &grafo[v] {
            let nova = distancia.checked_add(peso).ok_or("overflow da distancia")?;
            if distancias[u].is_none_or(|atual| nova < atual) {
                distancias[u] = Some(nova);
                heap.push(Reverse((nova, u)));
            }
        }
    }
    Ok(distancias)
}
fn main() {
    let grafo = [
        vec![(1, 4), (2, 1)],
        vec![(3, 1)],
        vec![(1, 2), (3, 5)],
        vec![],
        vec![],
    ];
    let distancias = dijkstra(&grafo, 0).unwrap();
    assert_eq!(distancias, vec![Some(0), Some(3), Some(1), Some(4), None]);
    println!("Distancias: {distancias:?}");
}
