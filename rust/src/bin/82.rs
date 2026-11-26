// Exercicio 82: Some duas matrizes de dimensoes iguais validando seu formato.
fn somar(a: &[Vec<i32>], b: &[Vec<i32>]) -> Option<Vec<Vec<i64>>> {
    let colunas = a.first().map_or(0, Vec::len);
    if a.len() != b.len() || a.iter().chain(b).any(|linha| linha.len() != colunas) {
        return None;
    }
    Some(
        a.iter()
            .zip(b)
            .map(|(a, b)| {
                a.iter()
                    .zip(b)
                    .map(|(&x, &y)| i64::from(x) + i64::from(y))
                    .collect()
            })
            .collect(),
    )
}
fn main() {
    let resultado = somar(&[vec![1, 2]], &[vec![3, 4]]).unwrap();
    assert_eq!(resultado, vec![vec![4, 6]]);
    assert!(somar(&[vec![1]], &[]).is_none());
    println!("Soma: {resultado:?}");
}
