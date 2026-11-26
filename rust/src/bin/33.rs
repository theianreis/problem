// Exercicio 33: Some os algarismos de um numero natural.
fn somar_algarismos(mut numero: u64) -> u64 {
    let mut soma = 0;
    while numero > 0 {
        soma += numero % 10;
        numero /= 10;
    }
    soma
}
fn main() {
    assert_eq!(somar_algarismos(0), 0);
    assert_eq!(somar_algarismos(9021), 12);
    println!("Soma dos algarismos: {}", somar_algarismos(9021));
}
