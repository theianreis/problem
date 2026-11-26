"""
Problema 21 — Pontos individuais com jitter
Área: seaborn | Nível: intermediário

Enunciado:
Use stripplot para mostrar todas as notas por turma com jitter e transparência.

Critérios de conclusão:
Mostre doze observações sem agregação e mantenha as categorias A e B na ordem dada.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'turma': ['A'] * 6 + ['B'] * 6, 'nota': [5, 6, 6, 7, 7, 8, 6, 7, 7, 8, 8, 9]})

# TODO: implemente a resolução deste problema.
