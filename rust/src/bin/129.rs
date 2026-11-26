// Exercicio 129: Implemente heapsort no proprio vetor em O(n log n).
fn descer(valores: &mut [i32], mut raiz: usize, fim: usize) {
    while 2 * raiz + 1 < fim {
        let mut filho = 2 * raiz + 1;
        if filho + 1 < fim && valores[filho + 1] > valores[filho] {
            filho += 1;
        }
        if valores[raiz] >= valores[filho] {
            break;
        }
        valores.swap(raiz, filho);
        raiz = filho;
    }
}
fn heapsort(valores: &mut [i32]) {
    let n = valores.len();
    for raiz in (0..n / 2).rev() {
        descer(valores, raiz, n);
    }
    for fim in (1..n).rev() {
        valores.swap(0, fim);
        descer(valores, 0, fim);
    }
}
fn main() {
    let mut valores = [4, -2, 7, 1, 4];
    heapsort(&mut valores);
    assert_eq!(valores, [-2, 1, 4, 4, 7]);
    heapsort(&mut []);
    println!("Heapsort: {valores:?}");
}
