// Exercicio 74: Inverta a ordem das palavras e normalize os espacos entre elas.
fn inverter_palavras(texto: &str) -> String {
    texto.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}
fn main() {
    assert_eq!(inverter_palavras("  eu estudo Rust "), "Rust estudo eu");
    assert_eq!(inverter_palavras(""), "");
    println!("{}", inverter_palavras("eu estudo Rust"));
}
