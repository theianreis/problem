// Exercicio 115: Resolva two sum em vetor nao ordenado com HashMap em tempo medio O(n).
use std::collections::HashMap;
fn two_sum(valores: &[i32], alvo: i64) -> Option<(usize, usize)> {
    let mut posicoes = HashMap::new();
    for (i, &valor) in valores.iter().enumerate() {
        if let Some(complemento) = alvo.checked_sub(i64::from(valor)) {
            if let Some(&j) = posicoes.get(&complemento) {
                return Some((j, i));
            }
        }
        posicoes.insert(i64::from(valor), i);
    }
    None
}
fn main() {
    assert_eq!(two_sum(&[3, 2, 4], 6), Some((1, 2)));
    assert_eq!(two_sum(&[3, 3], 6), Some((0, 1)));
    assert_eq!(two_sum(&[3], 6), None);
    println!("Indices: {:?}", two_sum(&[3, 2, 4], 6));
}
