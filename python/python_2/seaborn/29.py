"""
Problema 29 — Matriz de confusão normalizada
Área: seaborn | Nível: intermediário

Enunciado:
Normalize cada linha da matriz de confusão e mostre percentuais em heatmap.

Critérios de conclusão:
Cada linha deve somar 100%; os eixos devem identificar classe real e prevista.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

matriz = pd.DataFrame([[40, 10], [5, 45]], index=['Negativo', 'Positivo'], columns=['Negativo', 'Positivo'])

# TODO: implemente a resolução deste problema.
