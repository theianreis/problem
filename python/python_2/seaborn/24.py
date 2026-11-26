"""
Problema 24 — Estimativas em pontos
Área: seaborn | Nível: intermediário

Enunciado:
Use pointplot para comparar as médias por tratamento, com barras de um desvio padrão.

Critérios de conclusão:
As médias devem ser 10, 15 e 20; identifique as barras como desvio padrão.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'tratamento': ['Controle'] * 3 + ['A'] * 3 + ['B'] * 3, 'resultado': [8, 10, 12, 12, 15, 18, 16, 20, 24]})

# TODO: implemente a resolução deste problema.
