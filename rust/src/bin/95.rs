// Exercicio 95: Gere permutacoes distintas por trocas e backtracking.
use std::collections::HashSet;
fn permutar(valores: &mut [i32], inicio: usize, resultado: &mut Vec<Vec<i32>>) {
    if inicio == valores.len() {
        resultado.push(valores.to_vec());
        return;
    }
    let mut usados = HashSet::new();
    for i in inicio..valores.len() {
        if !usados.insert(valores[i]) {
            continue;
        }
        valores.swap(inicio, i);
        permutar(valores, inicio + 1, resultado);
        valores.swap(inicio, i);
    }
}
fn main() {
    let mut resultado = Vec::new();
    permutar(&mut [1, 1, 2], 0, &mut resultado);
    assert_eq!(resultado.len(), 3);
    assert!(resultado.contains(&vec![2, 1, 1]));
    println!("Permutacoes: {resultado:?}");
}
