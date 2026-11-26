// Exercicio 58: Calcule a intersecao de dois vetores sem repeticoes.
use std::collections::BTreeSet;
fn intersecao(a: &[i32], b: &[i32]) -> Vec<i32> {
    let a: BTreeSet<_> = a.iter().copied().collect();
    let b: BTreeSet<_> = b.iter().copied().collect();
    a.intersection(&b).copied().collect()
}
fn main() {
    let resultado = intersecao(&[3, 1, 3, 2], &[3, 3, 2, 7]);
    assert_eq!(resultado, vec![2, 3]);
    println!("Intersecao: {resultado:?}");
}
