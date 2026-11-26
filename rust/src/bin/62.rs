// Exercicio 62: Implemente busca binaria em uma fatia previamente ordenada.
fn busca_binaria(valores: &[i32], alvo: i32) -> Option<usize> {
    let (mut inicio, mut fim) = (0, valores.len());
    // Intervalo semiaberto [inicio, fim), inclusive para a fatia vazia.
    while inicio < fim {
        let meio = inicio + (fim - inicio) / 2;
        match valores[meio].cmp(&alvo) {
            std::cmp::Ordering::Equal => return Some(meio),
            std::cmp::Ordering::Less => inicio = meio + 1,
            std::cmp::Ordering::Greater => fim = meio,
        }
    }
    None
}
fn main() {
    assert_eq!(busca_binaria(&[1, 3, 5, 7], 5), Some(2));
    assert_eq!(busca_binaria(&[1, 3], 2), None);
    assert_eq!(busca_binaria(&[], 1), None);
    println!("Posicao: {:?}", busca_binaria(&[1, 3, 5, 7], 5));
}
