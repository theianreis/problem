// Exercicio 71: Verifique palindromos ignorando pontuacao e caixa, sem remover acentos.
fn palindromo(texto: &str) -> bool {
    let letras: Vec<char> = texto
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect();
    letras.iter().eq(letras.iter().rev())
}
fn main() {
    assert!(palindromo("A sacada da casa"));
    assert!(palindromo("Á, á!"));
    assert!(!palindromo("Rust"));
    println!("E palindromo: {}", palindromo("A sacada da casa"));
}
