"""
Problema 41 — Densidade bidimensional
Área: seaborn | Nível: avançado

Enunciado:
Use kdeplot com x, y, fill=True e níveis de densidade para duas medidas contínuas.

Critérios de conclusão:
Sobreponha os pontos em baixa opacidade; use dados com variação em ambas as direções e evite eixo singular.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'x': [1, 1.5, 2, 2.2, 2.8, 3, 3.4, 4, 4.2, 5], 'y': [2, 1, 3, 2.5, 4, 3.1, 5, 3.8, 5.5, 4.5]})

# TODO: implemente a resolução deste problema.
