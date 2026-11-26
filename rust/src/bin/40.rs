// Exercicio 40: Calcule o n-esimo Fibonacci iterativamente, com F(0) = 0.
fn fibonacci(n: u32) -> Option<u64> {
    if n == 0 {
        return Some(0);
    }
    let (mut anterior, mut atual) = (0_u64, 1_u64);
    for _ in 1..n {
        (anterior, atual) = (atual, anterior.checked_add(atual)?);
    }
    Some(atual)
}
fn main() {
    assert_eq!(fibonacci(10), Some(55));
    assert_eq!(fibonacci(94), None);
    println!("F(20): {:?}", fibonacci(20));
}
