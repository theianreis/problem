"""
Problema 37 — Variação por produto
Área: pandas | Nível: avançado

Enunciado:
Ordene os registros por produto e data e calcule a variação percentual de preço dentro de cada produto.

Critérios de conclusão:
A segunda observação de A deve variar 10%; a de B, -20%; as primeiras devem ser ausentes.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

df = pd.DataFrame({'produto': ['A', 'B', 'A', 'B'], 'data': [1, 1, 2, 2], 'preco': [100, 50, 110, 40]})

# TODO: implemente a resolução deste problema.
