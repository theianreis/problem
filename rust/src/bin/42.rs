// Exercicio 42: Converta um numero decimal para binario por divisoes sucessivas.
fn binario(mut numero: u64) -> String {
    if numero == 0 {
        return "0".into();
    }
    let mut bits = Vec::new();
    while numero > 0 {
        bits.push(if numero % 2 == 0 { '0' } else { '1' });
        numero /= 2;
    }
    bits.iter().rev().collect()
}
fn main() {
    assert_eq!(binario(0), "0");
    assert_eq!(binario(13), "1101");
    println!("13 em binario: {}", binario(13));
}
