// Exercicio 75: Detecte anagramas ignorando caixa e espacos, preservando acentos.
fn assinatura(texto: &str) -> Vec<char> {
    let mut letras: Vec<char> = texto
        .chars()
        .filter(|c| !c.is_whitespace())
        .flat_map(char::to_lowercase)
        .collect();
    letras.sort_unstable();
    letras
}
fn anagramas(a: &str, b: &str) -> bool {
    assinatura(a) == assinatura(b)
}
fn main() {
    assert!(anagramas("Amor", "Roma"));
    assert!(!anagramas("Rust", "Rato"));
    println!("Amor e Roma sao anagramas: {}", anagramas("Amor", "Roma"));
}
