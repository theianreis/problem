// Exercicio 106: Use um deque para verificar palindromos comparando suas duas pontas.
use std::collections::VecDeque;
fn palindromo(texto: &str) -> bool {
    let mut deque: VecDeque<char> = texto.chars().collect();
    while deque.len() > 1 {
        if deque.pop_front() != deque.pop_back() {
            return false;
        }
    }
    true
}
fn main() {
    assert!(palindromo("arara"));
    assert!(palindromo("été"));
    assert!(!palindromo("rust"));
    println!("arara e palindromo: {}", palindromo("arara"));
}
