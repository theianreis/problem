"""
Problema 44 — Paleta categórica consistente
Área: seaborn | Nível: avançado

Enunciado:
Compare médias em barras e observações em dispersão usando um dicionário de cores compartilhado por equipe.

Critérios de conclusão:
A e B devem manter suas cores entre os dois subplots; use hue explicitamente ao aplicar a paleta.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'equipe': ['A', 'A', 'B', 'B'], 'horas': [1, 2, 1, 2], 'resultado': [10, 15, 12, 18]})
paleta = {'A': '#0072B2', 'B': '#D55E00'}

# TODO: implemente a resolução deste problema.
