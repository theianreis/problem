// Exercicio 87: Rotacione uma matriz quadrada 90 graus para a direita no proprio lugar.
fn rotacionar(matriz: &mut [Vec<i32>]) -> bool {
    let n = matriz.len();
    if matriz.iter().any(|linha| linha.len() != n) {
        return false;
    }
    for i in 0..n {
        for j in i + 1..n {
            let temporario = matriz[i][j];
            matriz[i][j] = matriz[j][i];
            matriz[j][i] = temporario;
        }
    }
    for linha in matriz {
        linha.reverse();
    }
    true
}
fn main() {
    let mut matriz = vec![vec![1, 2], vec![3, 4]];
    assert!(rotacionar(&mut matriz));
    assert_eq!(matriz, vec![vec![3, 1], vec![4, 2]]);
    println!("Rotacionada: {matriz:?}");
}
