// Exercicio 103: Avalie uma expressao posfixa de inteiros com uma pilha e operacoes verificadas.
fn avaliar(expressao: &str) -> Result<i64, &'static str> {
    let mut pilha = Vec::<i64>::new();
    for token in expressao.split_whitespace() {
        if let Ok(numero) = token.parse() {
            pilha.push(numero);
            continue;
        }
        if !matches!(token, "+" | "-" | "*" | "/") {
            return Err("token invalido");
        }
        let b = pilha.pop().ok_or("faltou operando")?;
        let a = pilha.pop().ok_or("faltou operando")?;
        let resultado = match token {
            "+" => a.checked_add(b),
            "-" => a.checked_sub(b),
            "*" => a.checked_mul(b),
            _ => a.checked_div(b),
        }
        .ok_or("overflow ou divisao por zero")?;
        pilha.push(resultado);
    }
    if pilha.len() != 1 {
        return Err("quantidade invalida de operandos");
    }
    Ok(pilha[0])
}
fn main() {
    assert_eq!(avaliar("12 3 4 2 - * +"), Ok(18));
    assert!(avaliar("1 0 /").is_err());
    assert!(avaliar("1 2").is_err());
    println!("Resultado: {:?}", avaliar("12 3 4 2 - * +"));
}
