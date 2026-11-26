// Exercicio 29: Valide tres lados inteiros e classifique o triangulo.
fn triangulo(a: u32, b: u32, c: u32) -> Option<&'static str> {
    let mut lados = [u64::from(a), u64::from(b), u64::from(c)];
    lados.sort_unstable();
    if lados[0] == 0 || lados[0] + lados[1] <= lados[2] {
        return None;
    }
    Some(if a == b && b == c {
        "equilatero"
    } else if a == b || a == c || b == c {
        "isosceles"
    } else {
        "escaleno"
    })
}
fn main() {
    assert_eq!(triangulo(1, 2, 3), None);
    assert_eq!(triangulo(3, 3, 3), Some("equilatero"));
    println!("Triangulo: {:?}", triangulo(3, 4, 5));
}
