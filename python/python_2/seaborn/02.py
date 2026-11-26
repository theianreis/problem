"""
Problema 02 — Dispersão por turma
Área: seaborn | Nível: básico

Enunciado:
Compare a relação entre horas e nota distinguindo a turma por hue.

Critérios de conclusão:
Use uma legenda com A e B e mantenha as duas turmas nos mesmos eixos.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'horas': [1, 2, 3, 1, 2, 3], 'nota': [4, 6, 8, 5, 6, 9], 'turma': ['A'] * 3 + ['B'] * 3})

# TODO: implemente a resolução deste problema.
