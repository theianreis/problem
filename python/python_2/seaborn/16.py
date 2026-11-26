"""
Problema 16 — Barras de medianas
Área: seaborn | Nível: básico

Enunciado:
Use barplot com estimator='median' e errorbar=None para comparar tempos típicos por equipe.

Critérios de conclusão:
As barras devem estar em 12 para A e 15 para B; o outlier 100 não deve dominar a medida.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'equipe': ['A'] * 5 + ['B'] * 5, 'tempo': [10, 11, 12, 13, 100, 13, 14, 15, 16, 17]})

# TODO: implemente a resolução deste problema.
