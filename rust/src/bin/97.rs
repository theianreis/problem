// Exercicio 97: Encontre um caminho no labirinto com backtracking; true indica passagem.
fn buscar<const L: usize, const C: usize>(
    mapa: &[[bool; C]; L],
    posicao: (usize, usize),
    destino: (usize, usize),
    visitados: &mut [[bool; C]; L],
    caminho: &mut Vec<(usize, usize)>,
) -> bool {
    let (l, c) = posicao;
    if l >= L || c >= C || !mapa[l][c] || visitados[l][c] {
        return false;
    }
    visitados[l][c] = true;
    caminho.push(posicao);
    if posicao == destino {
        return true;
    }
    for (dl, dc) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        if let (Some(nl), Some(nc)) = (l.checked_add_signed(dl), c.checked_add_signed(dc)) {
            if buscar(mapa, (nl, nc), destino, visitados, caminho) {
                return true;
            }
        }
    }
    caminho.pop();
    false
}
fn main() {
    let mapa = [
        [true, false, true],
        [true, true, true],
        [false, false, true],
    ];
    let mut caminho = Vec::new();
    assert!(buscar(
        &mapa,
        (0, 0),
        (2, 2),
        &mut [[false; 3]; 3],
        &mut caminho
    ));
    assert_eq!(caminho, vec![(0, 0), (1, 0), (1, 1), (1, 2), (2, 2)]);
    println!("Caminho: {caminho:?}");
}
