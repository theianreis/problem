"""
Problema 20 — Violino dividido
Área: seaborn | Nível: intermediário

Enunciado:
Compare tempos de dois turnos dentro de cada loja usando violinplot com split=True.

Critérios de conclusão:
Cada loja deve ter um único violino dividido em duas metades identificadas por turno.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'loja': ['A'] * 12 + ['B'] * 12, 'turno': (['manhã'] * 6 + ['tarde'] * 6) * 2, 'tempo': [10, 11, 12, 13, 14, 15, 12, 13, 14, 15, 16, 17, 8, 9, 10, 11, 12, 13, 11, 12, 13, 14, 15, 16]})

# TODO: implemente a resolução deste problema.
