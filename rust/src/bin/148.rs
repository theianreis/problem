// Exercicio 148: Calcule distancia de edicao Levenshtein entre sequencias de caracteres Unicode.
fn distancia(a: &str, b: &str) -> usize {
    let b: Vec<char> = b.chars().collect();
    let mut anterior: Vec<usize> = (0..=b.len()).collect();
    for (i, letra_a) in a.chars().enumerate() {
        let mut atual = vec![i + 1; b.len() + 1];
        for (j, &letra_b) in b.iter().enumerate() {
            let substituir = anterior[j] + usize::from(letra_a != letra_b);
            let inserir = atual[j] + 1;
            let remover = anterior[j + 1] + 1;
            atual[j + 1] = substituir.min(inserir).min(remover);
        }
        anterior = atual;
    }
    anterior[b.len()]
}
fn main() {
    assert_eq!(distancia("kitten", "sitting"), 3);
    assert_eq!(distancia("", "ação"), 4);
    assert_eq!(distancia("café", "cafe"), 1);
    println!("Distancia de edicao: {}", distancia("kitten", "sitting"));
}
