// Exercicio 79: Busque uma substring e retorne seu indice em bytes usando comparacao direta.
fn buscar(texto: &str, padrao: &str) -> Option<usize> {
    if padrao.is_empty() {
        return Some(0);
    }
    if padrao.len() > texto.len() {
        return None;
    }
    texto
        .as_bytes()
        .windows(padrao.len())
        .position(|janela| janela == padrao.as_bytes())
}
fn main() {
    assert_eq!(buscar("olá Rust", "Rust"), Some(5));
    assert_eq!(buscar("abc", "abcd"), None);
    assert_eq!(buscar("", ""), Some(0));
    println!("Indice em bytes: {:?}", buscar("olá Rust", "Rust"));
}
