// Exercicio 55: Remova duplicatas sem alterar a ordem da primeira ocorrencia.
use std::collections::HashSet;
fn unicos(valores: &[i32]) -> Vec<i32> {
    let mut vistos = HashSet::new();
    valores
        .iter()
        .copied()
        .filter(|n| vistos.insert(*n))
        .collect()
}
fn main() {
    let resultado = unicos(&[3, 1, 3, 2, 1]);
    assert_eq!(resultado, vec![3, 1, 2]);
    println!("Sem duplicatas: {resultado:?}");
}
