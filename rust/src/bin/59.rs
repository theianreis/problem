// Exercicio 59: Calcule a uniao ordenada de dois vetores sem repeticoes.
use std::collections::BTreeSet;
fn uniao(a: &[i32], b: &[i32]) -> Vec<i32> {
    a.iter()
        .chain(b)
        .copied()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}
fn main() {
    let resultado = uniao(&[3, 1, 3], &[2, 1]);
    assert_eq!(resultado, vec![1, 2, 3]);
    println!("Uniao: {resultado:?}");
}
