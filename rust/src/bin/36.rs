// Exercicio 36: Verifique primalidade testando divisores ate a raiz quadrada.
fn primo(numero: u64) -> bool {
    if numero < 2 {
        return false;
    }
    let mut divisor = 2;
    // A divisao evita overflow que poderia ocorrer em divisor * divisor.
    while divisor <= numero / divisor {
        if numero % divisor == 0 {
            return false;
        }
        divisor += 1;
    }
    true
}
fn main() {
    assert!(!primo(1));
    assert!(!primo(49));
    assert!(primo(97));
    println!("97 e primo: {}", primo(97));
}
