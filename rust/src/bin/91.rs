// Exercicio 91: Calcule fatorial recursivo e retorne None quando nao couber em u128.
fn fatorial(n: u32) -> Option<u128> {
    // 34! cabe em u128; 35! nao cabe. O limite tambem evita recursao excessiva.
    if n > 34 {
        return None;
    }
    match n {
        0 | 1 => Some(1),
        _ => u128::from(n).checked_mul(fatorial(n - 1)?),
    }
}
fn main() {
    assert_eq!(fatorial(0), Some(1));
    assert_eq!(fatorial(5), Some(120));
    assert_eq!(fatorial(35), None);
    println!("10! = {:?}", fatorial(10));
}
