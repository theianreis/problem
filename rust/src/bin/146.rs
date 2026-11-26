// Exercicio 146: Pinte uma regiao conectada de uma imagem com flood fill iterativo.
fn preencher(imagem: &mut [Vec<u8>], linha: usize, coluna: usize, nova: u8) -> Option<usize> {
    let colunas = imagem.first()?.len();
    if linha >= imagem.len() || coluna >= colunas || imagem.iter().any(|l| l.len() != colunas) {
        return None;
    }
    let antiga = imagem[linha][coluna];
    if antiga == nova {
        return Some(0);
    }
    let mut pilha = vec![(linha, coluna)];
    imagem[linha][coluna] = nova;
    let mut pintados = 0;
    while let Some((l, c)) = pilha.pop() {
        pintados += 1;
        for (dl, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if let (Some(nl), Some(nc)) = (l.checked_add_signed(dl), c.checked_add_signed(dc)) {
                if nl < imagem.len() && nc < colunas && imagem[nl][nc] == antiga {
                    imagem[nl][nc] = nova;
                    pilha.push((nl, nc));
                }
            }
        }
    }
    Some(pintados)
}
fn main() {
    let mut imagem = vec![vec![1, 1, 0], vec![1, 0, 0], vec![0, 0, 1]];
    assert_eq!(preencher(&mut imagem, 0, 0, 2), Some(3));
    assert_eq!(imagem, vec![vec![2, 2, 0], vec![2, 0, 0], vec![0, 0, 1]]);
    assert_eq!(preencher(&mut imagem, 0, 0, 2), Some(0));
    println!("Imagem preenchida: {imagem:?}");
}
