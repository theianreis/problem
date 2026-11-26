"""
Problema 10 — Densidade por grupo
Área: seaborn | Nível: básico

Enunciado:
Compare distribuições de nota por turma com kdeplot e normalização independente.

Critérios de conclusão:
Use hue, common_norm=False e cut=0; mantenha a mesma escala de x.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'nota': [4, 5, 5.5, 6, 6.5, 7, 6, 7, 7.5, 8, 8.5, 9], 'turma': ['A'] * 6 + ['B'] * 6})

# TODO: implemente a resolução deste problema.
