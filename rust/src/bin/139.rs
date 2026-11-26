// Exercicio 139: Calcule distancias entre todos os pares de vertices com Floyd-Warshall.
fn floyd_warshall(
    n: usize,
    arestas: &[(usize, usize, i64)],
) -> Result<Vec<Vec<Option<i64>>>, &'static str> {
    let mut distancias = vec![vec![None; n]; n];
    for (i, linha) in distancias.iter_mut().enumerate() {
        linha[i] = Some(0_i64);
    }
    for &(a, b, peso) in arestas {
        if a >= n || b >= n {
            return Err("vertice invalido");
        }
        if distancias[a][b].is_none_or(|atual| peso < atual) {
            distancias[a][b] = Some(peso);
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if let (Some(a), Some(b)) = (distancias[i][k], distancias[k][j]) {
                    let nova = a.checked_add(b).ok_or("overflow da distancia")?;
                    if distancias[i][j].is_none_or(|atual| nova < atual) {
                        distancias[i][j] = Some(nova);
                    }
                }
            }
        }
    }
    if (0..n).any(|i| distancias[i][i].is_some_and(|d| d < 0)) {
        return Err("ciclo negativo");
    }
    Ok(distancias)
}
fn main() {
    let distancias = floyd_warshall(3, &[(0, 1, 3), (1, 2, 2), (0, 2, 10)]).unwrap();
    assert_eq!(distancias[0][2], Some(5));
    assert_eq!(distancias[2][0], None);
    assert!(floyd_warshall(1, &[(0, 0, -1)]).is_err());
    println!("Matriz de distancias: {distancias:?}");
}
