// Exercicio 44: Verifique se um numero e perfeito: igual a soma de seus divisores proprios.
fn perfeito(numero: u32) -> bool {
    if numero < 2 {
        return false;
    }
    let soma: u64 = (1..=numero / 2)
        .filter(|divisor| numero % divisor == 0)
        .map(u64::from)
        .sum();
    soma == u64::from(numero)
}
fn main() {
    assert!(perfeito(6));
    assert!(perfeito(28));
    assert!(!perfeito(1));
    println!("28 e perfeito: {}", perfeito(28));
}
