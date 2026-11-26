// Exercicio 64: Ordene com selection sort selecionando o menor restante.
fn selection_sort(valores: &mut [i32]) {
    for i in 0..valores.len() {
        let mut menor = i;
        for j in i + 1..valores.len() {
            if valores[j] < valores[menor] {
                menor = j;
            }
        }
        valores.swap(i, menor);
    }
}
fn main() {
    let mut valores = [9, -1, 3, 3, 0];
    selection_sort(&mut valores);
    assert_eq!(valores, [-1, 0, 3, 3, 9]);
    println!("Selection sort: {valores:?}");
}
