// Exercicio 102: Converta expressao infixa para posfixa com pilha de operadores.
// Aceita inteiros nao negativos, + - * / e parenteses. Nao aceita operadores unarios.
fn precedencia(op: &str) -> u8 {
    match op {
        "+" | "-" => 1,
        "*" | "/" => 2,
        _ => 0,
    }
}
fn posfixa(expressao: &str) -> Result<String, &'static str> {
    let mut tokens = Vec::new();
    let mut caracteres = expressao.chars().peekable();
    while let Some(c) = caracteres.next() {
        if c.is_whitespace() {
            continue;
        }
        if c.is_ascii_digit() {
            let mut numero = c.to_string();
            while caracteres.peek().is_some_and(char::is_ascii_digit) {
                numero.push(caracteres.next().unwrap());
            }
            tokens.push(numero);
        } else if "+-*/()".contains(c) {
            tokens.push(c.to_string());
        } else {
            return Err("caractere invalido");
        }
    }
    let (mut saida, mut operadores) = (Vec::new(), Vec::<String>::new());
    let mut espera_numero = true;
    for token in tokens {
        match token.as_str() {
            "(" if espera_numero => operadores.push(token),
            ")" if !espera_numero => {
                while operadores.last().is_some_and(|op| op != "(") {
                    saida.push(operadores.pop().unwrap());
                }
                if operadores.pop().is_none() {
                    return Err("parenteses sem abertura");
                }
            }
            "+" | "-" | "*" | "/" if !espera_numero => {
                while operadores
                    .last()
                    .is_some_and(|op| op != "(" && precedencia(op) >= precedencia(&token))
                {
                    saida.push(operadores.pop().unwrap());
                }
                operadores.push(token);
                espera_numero = true;
            }
            _ if espera_numero && token.bytes().all(|b| b.is_ascii_digit()) => {
                saida.push(token);
                espera_numero = false;
            }
            _ => return Err("ordem de tokens invalida"),
        }
    }
    if espera_numero {
        return Err("expressao incompleta");
    }
    while let Some(op) = operadores.pop() {
        if op == "(" {
            return Err("parenteses sem fechamento");
        }
        saida.push(op);
    }
    Ok(saida.join(" "))
}
fn main() {
    assert_eq!(posfixa("12 + 3 * (4 - 2)"), Ok("12 3 4 2 - * +".into()));
    assert!(posfixa("1 +").is_err());
    assert!(posfixa("(1 + 2").is_err());
    println!("Posfixa: {:?}", posfixa("12 + 3 * (4 - 2)"));
}
