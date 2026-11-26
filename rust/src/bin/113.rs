// Exercicio 113: Encontre a primeira repeticao em um fluxo de numeros com HashSet.
use std::collections::HashSet;
fn primeira_repeticao(valores: &[i32]) -> Option<i32> {
    let mut vistos = HashSet::new();
    valores.iter().copied().find(|valor| !vistos.insert(*valor))
}
fn main() {
    assert_eq!(primeira_repeticao(&[4, 2, 3, 2, 4]), Some(2));
    assert_eq!(primeira_repeticao(&[1, 2, 3]), None);
    println!(
        "Primeira repeticao: {:?}",
        primeira_repeticao(&[4, 2, 3, 2, 4])
    );
}
