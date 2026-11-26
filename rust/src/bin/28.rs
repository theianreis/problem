// Exercicio 28: Determine se um ano e bissexto pela regra do calendario gregoriano.
fn bissexto(ano: u32) -> bool {
    ano % 400 == 0 || (ano % 4 == 0 && ano % 100 != 0)
}
fn main() {
    assert!(bissexto(2000));
    assert!(!bissexto(1900));
    assert!(bissexto(2024));
    println!("2024 e bissexto: {}", bissexto(2024));
}
