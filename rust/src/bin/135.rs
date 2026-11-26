// Exercicio 135: Detecte ciclos em um grafo direcionado usando tres cores e DFS.
// Indices dos vizinhos devem estar dentro do grafo.
fn visitar(v: usize, grafo: &[Vec<usize>], cores: &mut [u8]) -> bool {
    cores[v] = 1; // Cinza: faz parte da pilha de recursao atual.
    for &u in &grafo[v] {
        if cores[u] == 1 || (cores[u] == 0 && visitar(u, grafo, cores)) {
            return true;
        }
    }
    cores[v] = 2; // Preto: todos os descendentes foram processados.
    false
}
fn tem_ciclo(grafo: &[Vec<usize>]) -> bool {
    let mut cores = vec![0; grafo.len()];
    (0..grafo.len()).any(|v| cores[v] == 0 && visitar(v, grafo, &mut cores))
}
fn main() {
    assert!(tem_ciclo(&[vec![1], vec![2], vec![0]]));
    assert!(!tem_ciclo(&[vec![1, 2], vec![2], vec![]]));
    println!("Ciclo detectado: {}", tem_ciclo(&[vec![1], vec![0]]));
}
