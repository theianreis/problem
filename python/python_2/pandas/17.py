"""
Problema 17 — Categorias ordenadas
Área: pandas | Nível: básico

Enunciado:
Transforme prioridade em uma categoria ordenada baixa < média < alta e ordene as tarefas.

Critérios de conclusão:
A ordem final das tarefas deve ser ['B', 'C', 'A']; o dtype deve ser categórico ordenado.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

df = pd.DataFrame({'tarefa': ['A', 'B', 'C'], 'prioridade': ['alta', 'baixa', 'média']})

# TODO: implemente a resolução deste problema.
