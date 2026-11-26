// Exercicio 98: Calcule o minimo de moedas para um valor com programacao dinamica.
fn minimo_moedas(moedas: &[usize], valor: usize) -> Option<usize> {
    let mut dp = vec![None; valor + 1];
    dp[0] = Some(0_usize);
    for soma in 1..=valor {
        for &moeda in moedas {
            if moeda == 0 || moeda > soma {
                continue;
            }
            if let Some(anterior) = dp[soma - moeda] {
                let candidato = anterior + 1;
                dp[soma] = Some(dp[soma].map_or(candidato, |atual| atual.min(candidato)));
            }
        }
    }
    dp[valor]
}
fn main() {
    assert_eq!(minimo_moedas(&[1, 3, 4], 6), Some(2));
    assert_eq!(minimo_moedas(&[2], 3), None);
    assert_eq!(minimo_moedas(&[], 0), Some(0));
    println!("Minimo de moedas: {:?}", minimo_moedas(&[1, 3, 4], 6));
}
