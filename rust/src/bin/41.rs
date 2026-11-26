// Exercicio 41: Calcule potencia inteira por exponenciacao binaria em O(log expoente).
fn potencia(mut base: u64, mut expoente: u32) -> Option<u64> {
    let mut resultado = 1_u64;
    while expoente > 0 {
        if expoente % 2 == 1 {
            resultado = resultado.checked_mul(base)?;
        }
        expoente /= 2;
        if expoente > 0 {
            base = base.checked_mul(base)?;
        }
    }
    Some(resultado)
}
fn main() {
    assert_eq!(potencia(2, 10), Some(1024));
    assert_eq!(potencia(u64::MAX, 0), Some(1));
    assert_eq!(potencia(2, 64), None);
    println!("3 elevado a 5: {:?}", potencia(3, 5));
}
