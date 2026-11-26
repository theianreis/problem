// Exercicio 37: Liste os primos ate um limite usando o crivo de Eratostenes.
fn crivo(limite: usize) -> Vec<usize> {
    if limite < 2 {
        return Vec::new();
    }
    let mut primos = vec![true; limite + 1];
    primos[0] = false;
    primos[1] = false;
    let mut p = 2;
    while p <= limite / p {
        if primos[p] {
            for multiplo in (p * p..=limite).step_by(p) {
                primos[multiplo] = false;
            }
        }
        p += 1;
    }
    (2..=limite).filter(|&n| primos[n]).collect()
}
fn main() {
    assert!(crivo(1).is_empty());
    assert_eq!(crivo(10), vec![2, 3, 5, 7]);
    println!("Primos: {:?}", crivo(50));
}
