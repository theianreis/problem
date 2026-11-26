// Exercicio 116: Conte subvetores com soma k usando prefixos e tabela de frequencias.
use std::collections::HashMap;
fn contar_subvetores(valores: &[i32], alvo: i64) -> u64 {
    let mut frequencias = HashMap::from([(0_i64, 1_u64)]);
    let (mut soma, mut total) = (0_i64, 0_u64);
    for &valor in valores {
        soma += i64::from(valor);
        if let Some(diferenca) = soma.checked_sub(alvo) {
            total += frequencias.get(&diferenca).copied().unwrap_or(0);
        }
        *frequencias.entry(soma).or_insert(0) += 1;
    }
    total
}
fn main() {
    assert_eq!(contar_subvetores(&[1, 1, 1], 2), 2);
    assert_eq!(contar_subvetores(&[1, -1, 0], 0), 3);
    println!("Subvetores: {}", contar_subvetores(&[1, -1, 0], 0));
}
