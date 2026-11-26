// Exercicio 47: Liste todos os divisores positivos de um numero em ordem crescente.
fn divisores(numero: u64) -> Vec<u64> {
    let mut resultado = Vec::new();
    let mut divisor = 1;
    while divisor <= numero / divisor {
        if numero % divisor == 0 {
            resultado.push(divisor);
            if divisor != numero / divisor {
                resultado.push(numero / divisor);
            }
        }
        divisor += 1;
    }
    resultado.sort_unstable();
    resultado
}
fn main() {
    // Para zero, retornamos vazio: seus divisores positivos sao infinitos.
    assert!(divisores(0).is_empty());
    assert_eq!(divisores(12), vec![1, 2, 3, 4, 6, 12]);
    println!("Divisores de 36: {:?}", divisores(36));
}
