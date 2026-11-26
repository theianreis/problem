// Exercicio 111: Crie uma agenda com HashMap: inserir, atualizar, consultar e remover.
use std::collections::HashMap;
fn main() {
    let mut agenda = HashMap::new();
    agenda.insert("Ana", "1111-1111");
    agenda.insert("Bia", "2222-2222");
    assert_eq!(agenda.insert("Ana", "3333-3333"), Some("1111-1111"));
    assert_eq!(agenda.get("Ana"), Some(&"3333-3333"));
    assert_eq!(agenda.remove("Bia"), Some("2222-2222"));
    assert!(!agenda.contains_key("Bia"));
    println!("Telefone de Ana: {}", agenda["Ana"]);
}
