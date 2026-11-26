// Exercicio 72: Conte vogais, consoantes ASCII e outros caracteres em um texto.
fn contar(texto: &str) -> (usize, usize, usize) {
    let (mut vogais, mut consoantes, mut outros) = (0, 0, 0);
    for letra in texto.chars() {
        match letra.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => vogais += 1,
            'a'..='z' => consoantes += 1,
            _ => outros += 1,
        }
    }
    (vogais, consoantes, outros)
}
fn main() {
    assert_eq!(contar("Rust 123!"), (1, 3, 5));
    println!(
        "Vogais, consoantes ASCII e outros: {:?}",
        contar("Rust 123!")
    );
}
