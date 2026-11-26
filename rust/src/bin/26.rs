// Exercicio 26: Encontre o maior de tres numeros sem ordenar.
fn maior(a: i32, b: i32, c: i32) -> i32 {
    let mut resultado = a;
    if b > resultado {
        resultado = b;
    }
    if c > resultado {
        resultado = c;
    }
    resultado
}
fn main() {
    assert_eq!(maior(-3, -1, -5), -1);
    println!("Maior: {}", maior(7, 12, 4));
}
