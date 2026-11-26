"""
Problema 06 — Coluna de faturamento
Área: pandas | Nível: básico

Enunciado:
Calcule o faturamento por item multiplicando quantidade por preço unitário, sem apply por linha.

Critérios de conclusão:
A coluna faturamento deve ser [20.0, 45.0, 12.0] e o total deve ser 77.0.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

df = pd.DataFrame({'produto': ['A', 'B', 'C'], 'quantidade': [2, 3, 4], 'preco': [10.0, 15.0, 3.0]})

# TODO: implemente a resolução deste problema.
