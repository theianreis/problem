"""
Problema 22 — Ranking por grupo
Área: pandas | Nível: intermediário

Enunciado:
Calcule o ranking decrescente de vendas em cada equipe com empates densos.

Critérios de conclusão:
Na equipe A, as vendas 100, 100 e 80 devem receber posições 1, 1 e 2.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

df = pd.DataFrame({'equipe': ['A', 'A', 'A', 'B'], 'vendas': [100, 100, 80, 50]})

# TODO: implemente a resolução deste problema.
