// Exercicio 94: Gere todos os subconjuntos por backtracking; use elementos distintos.
fn explorar(valores: &[i32], i: usize, atual: &mut Vec<i32>, resultado: &mut Vec<Vec<i32>>) {
    if i == valores.len() {
        resultado.push(atual.clone());
        return;
    }
    explorar(valores, i + 1, atual, resultado);
    atual.push(valores[i]);
    explorar(valores, i + 1, atual, resultado);
    atual.pop();
}
fn subconjuntos(valores: &[i32]) -> Vec<Vec<i32>> {
    let mut resultado = Vec::new();
    explorar(valores, 0, &mut Vec::new(), &mut resultado);
    resultado
}
fn main() {
    let resultado = subconjuntos(&[1, 2, 3]);
    assert_eq!(resultado.len(), 8);
    assert!(resultado.contains(&vec![]));
    assert!(resultado.contains(&vec![1, 2, 3]));
    println!("Subconjuntos: {resultado:?}");
}
