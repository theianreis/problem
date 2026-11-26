// Exercicio 88: Responda somas em intervalos [inicio, fim) usando somas prefixadas.
fn prefixos(valores: &[i32]) -> Vec<i64> {
    let mut somas = vec![0];
    for &valor in valores {
        somas.push(somas.last().unwrap() + i64::from(valor));
    }
    somas
}
fn consultar(somas: &[i64], inicio: usize, fim: usize) -> Option<i64> {
    if inicio > fim || fim >= somas.len() {
        return None;
    }
    Some(somas[fim] - somas[inicio])
}
fn main() {
    let somas = prefixos(&[2, 4, -1, 7]);
    assert_eq!(consultar(&somas, 1, 4), Some(10));
    assert_eq!(consultar(&somas, 2, 2), Some(0));
    assert_eq!(consultar(&somas, 0, 5), None);
    println!("Soma do intervalo: {:?}", consultar(&somas, 1, 4));
}
