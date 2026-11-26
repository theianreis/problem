// Exercicio 39: Calcule o minimo multiplo comum com deteccao de overflow.
fn mdc(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
fn mmc(a: u64, b: u64) -> Option<u64> {
    if a == 0 || b == 0 {
        return Some(0);
    }
    (a / mdc(a, b)).checked_mul(b)
}
fn main() {
    assert_eq!(mmc(12, 18), Some(36));
    assert_eq!(mmc(0, 0), Some(0));
    assert_eq!(mmc(u64::MAX, 2), None);
    println!("MMC: {:?}", mmc(12, 18));
}
