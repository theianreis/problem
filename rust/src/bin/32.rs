// Exercicio 32: Conte os algarismos de um inteiro sem converter para texto.
fn contar_algarismos(mut numero: u64) -> u32 {
    let mut quantidade = 1;
    while numero >= 10 {
        numero /= 10;
        quantidade += 1;
    }
    quantidade
}
fn main() {
    assert_eq!(contar_algarismos(0), 1);
    assert_eq!(contar_algarismos(12345), 5);
    println!("Algarismos: {}", contar_algarismos(12345));
}
