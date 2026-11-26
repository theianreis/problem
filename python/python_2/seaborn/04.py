"""
Problema 04 — Tamanho de pontos
Área: seaborn | Nível: básico

Enunciado:
Represente renda e expectativa de vida com tamanho proporcional à população.

Critérios de conclusão:
Configure size e sizes em scatterplot e mantenha uma legenda que explique os tamanhos.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'renda': [15, 25, 40, 60], 'expectativa': [65, 72, 78, 82], 'populacao': [10, 40, 20, 60]})

# TODO: implemente a resolução deste problema.
