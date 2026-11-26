// Exercicio 78: Valide o balanceamento de parenteses, colchetes e chaves com uma pilha.
fn balanceado(texto: &str) -> bool {
    let mut pilha = Vec::new();
    for caractere in texto.chars() {
        match caractere {
            '(' | '[' | '{' => pilha.push(caractere),
            ')' | ']' | '}' => {
                let abertura = match caractere {
                    ')' => '(',
                    ']' => '[',
                    _ => '{',
                };
                if pilha.pop() != Some(abertura) {
                    return false;
                }
            }
            _ => {}
        }
    }
    pilha.is_empty()
}
fn main() {
    assert!(balanceado("{[2 + (3 * 4)]}"));
    assert!(!balanceado("([)]"));
    assert!(!balanceado("("));
    println!("Balanceado: {}", balanceado("{[()]}"));
}
