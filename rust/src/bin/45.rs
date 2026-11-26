// Exercicio 45: Verifique numeros de Armstrong, como 153 = 1^3 + 5^3 + 3^3.
fn armstrong(numero: u32) -> bool {
    let texto = numero.to_string();
    let expoente = texto.len() as u32;
    let soma: u64 = texto
        .bytes()
        .map(|digito| u64::from(digito - b'0').pow(expoente))
        .sum();
    soma == u64::from(numero)
}
fn main() {
    assert!(armstrong(153));
    assert!(armstrong(0));
    assert!(!armstrong(154));
    println!("153 e Armstrong: {}", armstrong(153));
}
