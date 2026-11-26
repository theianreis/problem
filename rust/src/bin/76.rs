// Exercicio 76: Comprima repeticoes consecutivas em pares (caractere, quantidade).
fn comprimir(texto: &str) -> Vec<(char, usize)> {
    let mut grupos: Vec<(char, usize)> = Vec::new();
    for letra in texto.chars() {
        if let Some((anterior, quantidade)) = grupos.last_mut() {
            if *anterior == letra {
                *quantidade += 1;
                continue;
            }
        }
        grupos.push((letra, 1));
    }
    grupos
}
fn main() {
    assert_eq!(comprimir("aaabbáá"), vec![('a', 3), ('b', 2), ('á', 2)]);
    assert!(comprimir("").is_empty());
    println!("RLE: {:?}", comprimir("aaabbáá"));
}
