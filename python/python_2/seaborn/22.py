"""
Problema 22 — Enxame de pontos
Área: seaborn | Nível: intermediário

Enunciado:
Use swarmplot para mostrar tempos por equipe evitando sobreposição de marcadores.

Critérios de conclusão:
Cada uma das doze medições deve aparecer; ajuste o tamanho dos pontos se necessário.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'equipe': ['A'] * 6 + ['B'] * 6, 'tempo': [10, 10, 11, 11, 12, 12, 11, 11, 12, 12, 13, 13]})

# TODO: implemente a resolução deste problema.
