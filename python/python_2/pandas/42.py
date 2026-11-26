"""
Problema 42 — Amostragem estratificada
Área: pandas | Nível: avançado

Enunciado:
Sorteie duas linhas de cada classe com random_state=42, sem reposição.

Critérios de conclusão:
O resultado deve ter quatro linhas e duas observações de cada classe, sem ids repetidos.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

df = pd.DataFrame({'id': range(8), 'classe': ['A'] * 4 + ['B'] * 4, 'valor': range(10, 18)})

# TODO: implemente a resolução deste problema.
