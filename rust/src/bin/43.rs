// Exercicio 43: Converta texto binario para decimal validando digitos e overflow.
fn decimal(bits: &str) -> Option<u64> {
    if bits.is_empty() {
        return None;
    }
    let mut resultado = 0_u64;
    for bit in bits.chars() {
        let valor = match bit {
            '0' => 0,
            '1' => 1,
            _ => return None,
        };
        resultado = resultado.checked_mul(2)?.checked_add(valor)?;
    }
    Some(resultado)
}
fn main() {
    assert_eq!(decimal("1101"), Some(13));
    assert_eq!(decimal("102"), None);
    assert_eq!(decimal(""), None);
    println!("1101 em decimal: {:?}", decimal("1101"));
}
