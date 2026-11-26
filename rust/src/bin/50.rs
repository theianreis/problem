// Exercicio 50: Simule adivinhacao de um numero entre 1 e 100 usando busca binaria.
fn adivinhar(segredo: u32) -> Option<Vec<u32>> {
    if !(1..=100).contains(&segredo) {
        return None;
    }
    let (mut inicio, mut fim) = (1, 100);
    let mut tentativas = Vec::new();
    while inicio <= fim {
        let meio = inicio + (fim - inicio) / 2;
        tentativas.push(meio);
        match meio.cmp(&segredo) {
            std::cmp::Ordering::Equal => return Some(tentativas),
            std::cmp::Ordering::Less => inicio = meio + 1,
            std::cmp::Ordering::Greater => fim = meio - 1,
        }
    }
    None
}
fn main() {
    let tentativas = adivinhar(73).unwrap();
    assert_eq!(tentativas.last(), Some(&73));
    assert!(tentativas.len() <= 7);
    println!("Tentativas para 73: {tentativas:?}");
}
