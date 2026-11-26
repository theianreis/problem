// Exercicio 35: Verifique se um inteiro natural e palindromo.
fn palindromo(numero: u64) -> bool {
    let texto = numero.to_string();
    texto.bytes().eq(texto.bytes().rev())
}
fn main() {
    assert!(palindromo(12321));
    assert!(!palindromo(120));
    println!("12321 e palindromo: {}", palindromo(12321));
}
