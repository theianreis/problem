// Exercicio 126: Use BinaryHeap como fila de prioridade para atender tarefas urgentes primeiro.
use std::collections::BinaryHeap;
fn main() {
    // Tuplas desempataram pela descricao; maior numero significa maior prioridade.
    let mut tarefas = BinaryHeap::from([(2, "estudar"), (5, "corrigir falha"), (1, "organizar")]);
    assert_eq!(tarefas.peek(), Some(&(5, "corrigir falha")));
    let mut ordem = Vec::new();
    while let Some((_, tarefa)) = tarefas.pop() {
        ordem.push(tarefa);
    }
    assert_eq!(ordem, vec!["corrigir falha", "estudar", "organizar"]);
    println!("Ordem de atendimento: {ordem:?}");
}
