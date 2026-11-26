// Exercicio 119: Calcule diferenca e diferenca simetrica de conjuntos ordenados.
use std::collections::BTreeSet;
fn main() {
    let a = BTreeSet::from([1, 2, 3, 4]);
    let b = BTreeSet::from([3, 4, 5]);
    let diferenca: Vec<_> = a.difference(&b).copied().collect();
    let simetrica: Vec<_> = a.symmetric_difference(&b).copied().collect();
    assert_eq!(diferenca, vec![1, 2]);
    assert_eq!(simetrica, vec![1, 2, 5]);
    println!("A - B: {diferenca:?}; diferenca simetrica: {simetrica:?}");
}
