"""
Problema 18 — Boxplots agrupados
Área: seaborn | Nível: intermediário

Enunciado:
Compare notas por turma e turno usando boxplot com hue.

Critérios de conclusão:
Cada turma deve conter duas caixas, uma para manhã e outra para tarde.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'turma': ['A'] * 8 + ['B'] * 8, 'turno': (['manhã'] * 4 + ['tarde'] * 4) * 2, 'nota': [5, 6, 7, 8, 6, 7, 8, 9, 4, 5, 6, 7, 7, 8, 9, 10]})

# TODO: implemente a resolução deste problema.
