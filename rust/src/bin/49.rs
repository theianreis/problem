// Exercicio 49: Calcule as raizes reais de uma equacao quadratica com coeficientes finitos.
fn raizes(a: f64, b: f64, c: f64) -> Result<(f64, f64), &'static str> {
    if ![a, b, c].iter().all(|x| x.is_finite()) || a == 0.0 {
        return Err("coeficientes invalidos para uma equacao quadratica");
    }
    let delta = b * b - 4.0 * a * c;
    if !delta.is_finite() {
        return Err("calculo fora do intervalo numerico");
    }
    if delta < 0.0 {
        return Err("sem raizes reais");
    }
    Ok((
        (-b - delta.sqrt()) / (2.0 * a),
        (-b + delta.sqrt()) / (2.0 * a),
    ))
}
fn main() {
    assert_eq!(raizes(1.0, -5.0, 6.0), Ok((2.0, 3.0)));
    assert!(raizes(1.0, 0.0, 1.0).is_err());
    println!("Raizes: {:?}", raizes(1.0, -5.0, 6.0));
}
