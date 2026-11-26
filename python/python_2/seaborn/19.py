"""
Problema 19 — Violino com quartis
Área: seaborn | Nível: intermediário

Enunciado:
Mostre a distribuição de tempos por modalidade com violinplot e inner='quart'.

Critérios de conclusão:
Os dois violinos devem exibir quartis e usar cut=0 para não extrapolar os extremos observados.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'modalidade': ['online'] * 8 + ['presencial'] * 8, 'tempo': [10, 12, 13, 14, 15, 16, 18, 20, 15, 17, 18, 19, 20, 22, 25, 28]})

# TODO: implemente a resolução deste problema.
