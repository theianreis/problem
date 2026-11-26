// Exercicio 46: Gere uma sequencia de Collatz com limite de passos e deteccao de overflow.
fn collatz(mut numero: u64, limite: usize) -> Option<Vec<u64>> {
    if numero == 0 {
        return None;
    }
    let mut sequencia = vec![numero];
    for _ in 0..limite {
        if numero == 1 {
            return Some(sequencia);
        }
        numero = if numero % 2 == 0 {
            numero / 2
        } else {
            numero.checked_mul(3)?.checked_add(1)?
        };
        sequencia.push(numero);
    }
    if numero == 1 {
        Some(sequencia)
    } else {
        None
    }
}
fn main() {
    assert_eq!(collatz(6, 8), Some(vec![6, 3, 10, 5, 16, 8, 4, 2, 1]));
    assert_eq!(collatz(6, 2), None);
    println!("Collatz: {:?}", collatz(6, 100));
}
