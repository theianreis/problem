// Exercicio 34: Inverta os algarismos de um numero e detecte overflow.
fn inverter(mut numero: u64) -> Option<u64> {
    let mut invertido = 0_u64;
    while numero > 0 {
        invertido = invertido.checked_mul(10)?.checked_add(numero % 10)?;
        numero /= 10;
    }
    Some(invertido)
}
fn main() {
    assert_eq!(inverter(120), Some(21));
    assert_eq!(inverter(0), Some(0));
    assert_eq!(inverter(u64::MAX), None);
    println!("Invertido: {:?}", inverter(12345));
}
