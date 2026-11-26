// Exercicio 134: Identifique componentes conexas de um grafo nao direcionado.
// Precondicao: indices validos e cada aresta representada nas duas direcoes.
fn componentes(grafo: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut visitados = vec![false; grafo.len()];
    let mut grupos = Vec::new();
    for origem in 0..grafo.len() {
        if visitados[origem] {
            continue;
        }
        let mut pilha = vec![origem];
        visitados[origem] = true;
        let mut grupo = Vec::new();
        while let Some(v) = pilha.pop() {
            grupo.push(v);
            for &u in &grafo[v] {
                if !visitados[u] {
                    visitados[u] = true;
                    pilha.push(u);
                }
            }
        }
        grupo.sort_unstable();
        grupos.push(grupo);
    }
    grupos
}
fn main() {
    let grafo = vec![vec![1], vec![0], vec![3], vec![2], vec![]];
    let grupos = componentes(&grafo);
    assert_eq!(grupos, vec![vec![0, 1], vec![2, 3], vec![4]]);
    println!("Componentes: {grupos:?}");
}
