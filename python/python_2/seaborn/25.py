"""
Problema 25 — Ordem pela mediana
Área: seaborn | Nível: intermediário

Enunciado:
Calcule as medianas por equipe e use-as para ordenar um boxplot horizontal do menor para o maior.

Critérios de conclusão:
A ordem das equipes deve ser C, A, B, com medianas 5, 10 e 20.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'equipe': ['A'] * 3 + ['B'] * 3 + ['C'] * 3, 'tempo': [8, 10, 12, 18, 20, 22, 3, 5, 7]})

# TODO: implemente a resolução deste problema.
