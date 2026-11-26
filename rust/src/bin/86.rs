// Exercicio 86: Percorra uma matriz retangular em espiral no sentido horario.
fn espiral(matriz: &[Vec<i32>]) -> Option<Vec<i32>> {
    let colunas = matriz.first().map_or(0, Vec::len);
    if matriz.iter().any(|linha| linha.len() != colunas) {
        return None;
    }
    let (mut cima, mut baixo, mut esquerda, mut direita) = (0, matriz.len(), 0, colunas);
    let mut resultado = Vec::new();
    while cima < baixo && esquerda < direita {
        for j in esquerda..direita {
            resultado.push(matriz[cima][j]);
        }
        cima += 1;
        for linha in matriz.iter().take(baixo).skip(cima) {
            resultado.push(linha[direita - 1]);
        }
        direita -= 1;
        if cima < baixo && esquerda < direita {
            baixo -= 1;
            for j in (esquerda..direita).rev() {
                resultado.push(matriz[baixo][j]);
            }
            for i in (cima..baixo).rev() {
                resultado.push(matriz[i][esquerda]);
            }
            esquerda += 1;
        }
    }
    Some(resultado)
}
fn main() {
    let resultado = espiral(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]).unwrap();
    assert_eq!(resultado, vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    assert_eq!(espiral(&[vec![1], vec![2]]), Some(vec![1, 2]));
    println!("Espiral: {resultado:?}");
}
