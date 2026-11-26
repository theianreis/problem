"""
Problema 48 — Retenção por coorte
Área: pandas | Nível: avançado

Enunciado:
Defina a coorte de cada cliente pelo mês de sua primeira compra e conte clientes únicos por coorte e mês da compra.

Critérios de conclusão:
A coorte de janeiro deve ter 2 clientes em janeiro e 1 em fevereiro, resultando em retenção de 50% em fevereiro.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

df = pd.DataFrame({'cliente': ['A', 'B', 'A', 'C'], 'data': pd.to_datetime(['2026-01-01', '2026-01-20', '2026-02-05', '2026-02-10'])})

# TODO: implemente a resolução deste problema.
