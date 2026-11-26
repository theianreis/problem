"""
Problema 39 — Matriz de relações
Área: seaborn | Nível: avançado

Enunciado:
Use pairplot para inspecionar relações entre três medidas, distinguindo classes por hue.

Critérios de conclusão:
Inclua apenas as três colunas numéricas e use diag_kind='hist'; a legenda deve identificar duas classes.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'altura': [150, 155, 160, 165, 170, 175, 180, 185], 'peso': [50, 54, 57, 60, 67, 70, 78, 82], 'idade': [20, 22, 25, 24, 30, 32, 35, 38], 'classe': ['A'] * 4 + ['B'] * 4})

# TODO: implemente a resolução deste problema.
