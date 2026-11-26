// Exercicio 92: Calcule Fibonacci recursivo com memoizacao, evitando recalculos.
fn calcular(n: usize, memo: &mut [Option<u64>]) -> Option<u64> {
    if let Some(valor) = memo[n] {
        return Some(valor);
    }
    let valor = if n < 2 {
        n as u64
    } else {
        calcular(n - 1, memo)?.checked_add(calcular(n - 2, memo)?)?
    };
    memo[n] = Some(valor);
    Some(valor)
}
fn fibonacci(n: usize) -> Option<u64> {
    if n > 93 {
        return None;
    }
    calcular(n, &mut vec![None; n + 1])
}
fn main() {
    assert_eq!(fibonacci(10), Some(55));
    assert_eq!(fibonacci(94), None);
    println!("F(40): {:?}", fibonacci(40));
}
