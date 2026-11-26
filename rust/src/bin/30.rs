// Exercicio 30: Implemente uma calculadora com tratamento de divisao por zero.
fn calcular(a: f64, operador: char, b: f64) -> Result<f64, &'static str> {
    match operador {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' if b != 0.0 => Ok(a / b),
        '/' => Err("divisao por zero"),
        _ => Err("operador desconhecido"),
    }
}
fn main() {
    assert_eq!(calcular(8.0, '/', 2.0), Ok(4.0));
    assert!(calcular(1.0, '/', 0.0).is_err());
    println!("8 / 2 = {:?}", calcular(8.0, '/', 2.0));
}
