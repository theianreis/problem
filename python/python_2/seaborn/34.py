"""
Problema 34 — Regressão por grupo
Área: seaborn | Nível: intermediário

Enunciado:
Use lmplot com hue para comparar as relações entre horas e resultado em duas equipes.

Critérios de conclusão:
Mostre duas retas de regressão e uma legenda; use ci=None para focar nas tendências.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'horas': [1, 2, 3, 4] * 2, 'resultado': [2, 4, 6, 8, 3, 4, 5, 6], 'equipe': ['A'] * 4 + ['B'] * 4})

# TODO: implemente a resolução deste problema.
