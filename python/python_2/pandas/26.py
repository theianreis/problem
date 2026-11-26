"""
Problema 26 — Tabela dinâmica
Área: pandas | Nível: intermediário

Enunciado:
Monte uma pivot_table com regiões nas linhas, produtos nas colunas e soma de vendas; preencha combinações ausentes com zero.

Critérios de conclusão:
Sul/A deve valer 150 e Norte/B deve valer zero.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

df = pd.DataFrame({'regiao': ['Sul', 'Sul', 'Sul', 'Norte'], 'produto': ['A', 'A', 'B', 'A'], 'valor': [100, 50, 80, 40]})

# TODO: implemente a resolução deste problema.
