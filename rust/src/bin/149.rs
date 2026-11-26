// Exercicio 149: Selecione o maximo de atividades sem sobreposicao com um algoritmo guloso.
fn selecionar(atividades: &[(u32, u32)]) -> Option<Vec<(u32, u32)>> {
    if atividades.iter().any(|&(inicio, fim)| inicio >= fim) {
        return None;
    }
    let mut atividades = atividades.to_vec();
    atividades.sort_unstable_by_key(|&(inicio, fim)| (fim, inicio));
    let mut escolhidas = Vec::new();
    let mut ultimo_fim = 0;
    for (inicio, fim) in atividades {
        if inicio >= ultimo_fim {
            escolhidas.push((inicio, fim));
            ultimo_fim = fim;
        }
    }
    Some(escolhidas)
}
fn main() {
    // Intervalos [inicio, fim): uma atividade pode comecar quando a outra termina.
    let atividades = [(1, 4), (3, 5), (0, 6), (5, 7), (3, 9), (8, 11), (12, 16)];
    let escolhidas = selecionar(&atividades).unwrap();
    assert_eq!(escolhidas, vec![(1, 4), (5, 7), (8, 11), (12, 16)]);
    assert!(selecionar(&[(3, 2)]).is_none());
    println!("Atividades selecionadas: {escolhidas:?}");
}
