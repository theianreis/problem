"""
Problema 05 — Linha por categoria
Área: seaborn | Nível: básico

Enunciado:
Use lineplot para acompanhar vendas diárias por loja, sem agregação entre lojas.

Critérios de conclusão:
Mostre três pontos por loja e uma legenda identificando A e B.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'dia': [1, 2, 3, 1, 2, 3], 'venda': [10, 20, 15, 12, 18, 25], 'loja': ['A'] * 3 + ['B'] * 3})

# TODO: implemente a resolução deste problema.
