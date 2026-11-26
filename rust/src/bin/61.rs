// Exercicio 61: Implemente busca linear retornando a primeira posicao encontrada.
fn busca_linear(valores: &[i32], alvo: i32) -> Option<usize> {
    for (indice, &valor) in valores.iter().enumerate() {
        if valor == alvo {
            return Some(indice);
        }
    }
    None
}
fn main() {
    assert_eq!(busca_linear(&[8, 3, 8], 8), Some(0));
    assert_eq!(busca_linear(&[], 8), None);
    println!("Posicao: {:?}", busca_linear(&[8, 3, 5], 5));
}
