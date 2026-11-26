"""
Problema 41 — Outliers por grupo
Área: pandas | Nível: avançado

Enunciado:
Calcule limites de Tukey separadamente por categoria e marque valores fora de Q1 - 1.5*IQR e Q3 + 1.5*IQR.

Critérios de conclusão:
Apenas 100 na categoria A deve ser marcado; nenhuma observação de B deve ser marcada.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

df = pd.DataFrame({'categoria': ['A'] * 5 + ['B'] * 5, 'valor': [10, 11, 12, 13, 100, 20, 21, 22, 23, 24]})

# TODO: implemente a resolução deste problema.
