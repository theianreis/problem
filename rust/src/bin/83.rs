// Exercicio 83: Multiplique matrizes compativeis usando tres lacos.
fn multiplicar(a: &[Vec<i32>], b: &[Vec<i32>]) -> Option<Vec<Vec<i128>>> {
    let internas = a.first()?.len();
    let colunas = b.first()?.len();
    if internas != b.len()
        || a.iter().any(|l| l.len() != internas)
        || b.iter().any(|l| l.len() != colunas)
    {
        return None;
    }
    let mut resultado = vec![vec![0_i128; colunas]; a.len()];
    for i in 0..a.len() {
        for k in 0..internas {
            for j in 0..colunas {
                resultado[i][j] =
                    resultado[i][j].checked_add(i128::from(a[i][k]) * i128::from(b[k][j]))?;
            }
        }
    }
    Some(resultado)
}
fn main() {
    let a = [vec![1, 2], vec![3, 4]];
    let b = [vec![5, 6], vec![7, 8]];
    let resultado = multiplicar(&a, &b).unwrap();
    assert_eq!(resultado, vec![vec![19, 22], vec![43, 50]]);
    println!("Produto: {resultado:?}");
}
