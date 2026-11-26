// Exercicio 31: Some os multiplos de 3 ou 5 menores que um limite.
fn soma_multiplos(limite: u32) -> u64 {
    let mut soma = 0;
    for numero in 0..limite {
        if numero % 3 == 0 || numero % 5 == 0 {
            soma += u64::from(numero);
        }
    }
    soma
}
fn main() {
    assert_eq!(soma_multiplos(10), 23);
    println!("Soma abaixo de 1000: {}", soma_multiplos(1000));
}
