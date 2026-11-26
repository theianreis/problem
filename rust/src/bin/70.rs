// Exercicio 70: Encontre o k-esimo menor (k comeca em zero) usando quickselect.
fn quickselect(valores: &mut [i32], k: usize) -> Option<i32> {
    if k >= valores.len() {
        return None;
    }
    let ultimo = valores.len() - 1;
    let mut posicao = 0;
    for i in 0..ultimo {
        if valores[i] < valores[ultimo] {
            valores.swap(i, posicao);
            posicao += 1;
        }
    }
    valores.swap(posicao, ultimo);
    match k.cmp(&posicao) {
        std::cmp::Ordering::Equal => Some(valores[posicao]),
        std::cmp::Ordering::Less => quickselect(&mut valores[..posicao], k),
        std::cmp::Ordering::Greater => quickselect(&mut valores[posicao + 1..], k - posicao - 1),
    }
}
fn main() {
    let mut valores = [7, 1, 5, 3, 3];
    assert_eq!(quickselect(&mut valores, 2), Some(3));
    assert_eq!(quickselect(&mut [], 0), None);
    println!("Terceiro menor: {:?}", quickselect(&mut valores, 2));
}
