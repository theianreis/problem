// Exercicio 56: Conte a frequencia de cada numero e apresente as chaves em ordem.
use std::collections::BTreeMap;
fn frequencias(valores: &[i32]) -> BTreeMap<i32, usize> {
    let mut mapa = BTreeMap::new();
    for &valor in valores {
        *mapa.entry(valor).or_insert(0) += 1;
    }
    mapa
}
fn main() {
    let mapa = frequencias(&[4, 2, 4, 3, 2, 4]);
    assert_eq!(mapa.get(&4), Some(&3));
    assert_eq!(mapa.len(), 3);
    println!("Frequencias: {mapa:?}");
}
