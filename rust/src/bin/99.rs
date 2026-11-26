// Exercicio 99: Resolva a mochila 0/1: cada item (peso, valor) pode ser usado uma vez.
fn mochila(itens: &[(usize, u32)], capacidade: usize) -> u64 {
    let mut dp = vec![0_u64; capacidade + 1];
    for &(peso, valor) in itens {
        // Ordem decrescente impede reutilizar o mesmo item.
        for c in (peso..=capacidade).rev() {
            dp[c] = dp[c].max(dp[c - peso] + u64::from(valor));
        }
    }
    dp[capacidade]
}
fn main() {
    assert_eq!(mochila(&[(2, 3), (3, 4), (4, 5), (5, 6)], 5), 7);
    assert_eq!(mochila(&[(2, 3)], 4), 3);
    println!("Valor maximo: {}", mochila(&[(2, 3), (3, 4), (4, 5)], 5));
}
