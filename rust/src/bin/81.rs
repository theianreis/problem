// Exercicio 81: Transponha uma matriz retangular, rejeitando linhas de tamanhos diferentes.
fn transpor(matriz: &[Vec<i32>]) -> Option<Vec<Vec<i32>>> {
    let colunas = matriz.first().map_or(0, Vec::len);
    if matriz.iter().any(|linha| linha.len() != colunas) {
        return None;
    }
    let mut resultado = vec![vec![0; matriz.len()]; colunas];
    for (i, linha) in matriz.iter().enumerate() {
        for (j, &valor) in linha.iter().enumerate() {
            resultado[j][i] = valor;
        }
    }
    Some(resultado)
}
fn main() {
    let resultado = transpor(&[vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
    assert_eq!(resultado, vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
    assert!(transpor(&[vec![1], vec![]]).is_none());
    println!("Transposta: {resultado:?}");
}
