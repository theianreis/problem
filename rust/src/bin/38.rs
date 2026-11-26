// Exercicio 38: Calcule o maximo divisor comum pelo algoritmo de Euclides.
fn mdc(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
fn main() {
    assert_eq!(mdc(0, 0), 0);
    assert_eq!(mdc(48, 18), 6);
    println!("MDC: {}", mdc(48, 18));
}
