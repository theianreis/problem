// Exercicio 138: Use Bellman-Ford para pesos negativos e detecte ciclos negativos alcancaveis.
fn bellman_ford(
    n: usize,
    arestas: &[(usize, usize, i64)],
    origem: usize,
) -> Result<Vec<Option<i64>>, &'static str> {
    if origem >= n || arestas.iter().any(|&(a, b, _)| a >= n || b >= n) {
        return Err("vertice invalido");
    }
    let mut distancias = vec![None; n];
    distancias[origem] = Some(0_i64);
    for rodada in 0..n {
        let mut mudou = false;
        for &(a, b, peso) in arestas {
            if let Some(distancia) = distancias[a] {
                let nova = distancia.checked_add(peso).ok_or("overflow da distancia")?;
                if distancias[b].is_none_or(|atual| nova < atual) {
                    if rodada == n - 1 {
                        return Err("ciclo negativo alcancavel");
                    }
                    distancias[b] = Some(nova);
                    mudou = true;
                }
            }
        }
        if !mudou {
            break;
        }
    }
    Ok(distancias)
}
fn main() {
    let arestas = [(0, 1, 4), (0, 2, 5), (1, 2, -2)];
    assert_eq!(
        bellman_ford(3, &arestas, 0),
        Ok(vec![Some(0), Some(4), Some(2)])
    );
    assert!(bellman_ford(2, &[(0, 1, -1), (1, 0, -1)], 0).is_err());
    println!("Distancias: {:?}", bellman_ford(3, &arestas, 0));
}
