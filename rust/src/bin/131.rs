// Exercicio 131: Represente um grafo nao direcionado por listas de adjacencia e calcule graus.
fn construir(vertices: usize, arestas: &[(usize, usize)]) -> Option<Vec<Vec<usize>>> {
    let mut grafo = vec![Vec::new(); vertices];
    for &(a, b) in arestas {
        if a >= vertices || b >= vertices {
            return None;
        }
        grafo[a].push(b);
        grafo[b].push(a);
    }
    Some(grafo)
}
fn main() {
    let grafo = construir(4, &[(0, 1), (0, 2), (2, 3)]).unwrap();
    let graus: Vec<_> = grafo.iter().map(Vec::len).collect();
    assert_eq!(graus, vec![2, 1, 2, 1]);
    assert_eq!(graus.iter().sum::<usize>(), 6);
    assert!(construir(2, &[(0, 2)]).is_none());
    println!("Adjacencias: {grafo:?}; graus: {graus:?}");
}
