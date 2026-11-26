// Exercicio 112: Conte palavras com HashMap ignorando caixa e pontuacao.
use std::collections::HashMap;
fn contar(texto: &str) -> HashMap<String, usize> {
    let mut contagens = HashMap::new();
    for palavra in texto
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
    {
        *contagens.entry(palavra.to_lowercase()).or_insert(0) += 1;
    }
    contagens
}
fn main() {
    let contagens = contar("Rust, rust! Aprender Rust.");
    assert_eq!(contagens.get("rust"), Some(&3));
    let mut ordenadas: Vec<_> = contagens.iter().collect();
    ordenadas.sort_by_key(|(palavra, _)| *palavra);
    println!("Frequencias: {ordenadas:?}");
}
