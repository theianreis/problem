// Exercicio 93: Resolva a Torre de Hanoi e liste os movimentos para tres discos.
fn hanoi(n: u32, origem: char, auxiliar: char, destino: char, movimentos: &mut Vec<(char, char)>) {
    if n == 0 {
        return;
    }
    hanoi(n - 1, origem, destino, auxiliar, movimentos);
    movimentos.push((origem, destino));
    hanoi(n - 1, auxiliar, origem, destino, movimentos);
}
fn main() {
    // Custo exponencial: 2^n - 1 movimentos. Use poucos discos.
    let mut movimentos = Vec::new();
    hanoi(3, 'A', 'B', 'C', &mut movimentos);
    assert_eq!(movimentos.len(), 7);
    let mut torres = [vec![3, 2, 1], vec![], vec![]];
    for &(origem, destino) in &movimentos {
        let (a, b) = (
            (origem as u8 - b'A') as usize,
            (destino as u8 - b'A') as usize,
        );
        let disco = torres[a].pop().unwrap();
        assert!(torres[b].last().is_none_or(|&topo| topo > disco));
        torres[b].push(disco);
    }
    assert_eq!(torres[2], vec![3, 2, 1]);
    println!("Movimentos: {movimentos:?}");
}
