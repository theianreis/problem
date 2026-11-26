// Exercicio 96: Resolva N rainhas sem conflitos de coluna ou diagonal.
fn explorar(n: usize, colunas: &mut Vec<usize>, solucoes: &mut Vec<Vec<usize>>) {
    let linha = colunas.len();
    if linha == n {
        solucoes.push(colunas.clone());
        return;
    }
    for coluna in 0..n {
        if colunas
            .iter()
            .enumerate()
            .all(|(l, &c)| c != coluna && c.abs_diff(coluna) != linha - l)
        {
            colunas.push(coluna);
            explorar(n, colunas, solucoes);
            colunas.pop();
        }
    }
}
fn main() {
    let mut solucoes = Vec::new();
    explorar(4, &mut Vec::new(), &mut solucoes);
    assert_eq!(solucoes.len(), 2);
    assert!(solucoes.contains(&vec![1, 3, 0, 2]));
    println!("Coluna de cada rainha por linha: {solucoes:?}");
}
