// Exercicio 73: Conte palavras separadas por espacos, tabulacoes ou quebras de linha.
fn contar_palavras(texto: &str) -> usize {
    texto.split_whitespace().count()
}
fn main() {
    assert_eq!(contar_palavras("  Rust  e muito legal  "), 4);
    assert_eq!(contar_palavras(""), 0);
    println!(
        "Quantidade de palavras: {}",
        contar_palavras("estudando Rust todos os dias")
    );
}
