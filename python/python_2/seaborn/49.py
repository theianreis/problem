"""
Problema 49 — Bootstrap de médias
Área: seaborn | Nível: avançado

Enunciado:
Use barplot com média e intervalo de confiança bootstrap de 95%, fixando seed=42 e n_boot=2000.

Critérios de conclusão:
Mostre duas barras e intervalos reproduzíveis; identifique explicitamente que representam confiança da média, não desvio padrão.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'grupo': ['A'] * 8 + ['B'] * 8, 'valor': [8, 9, 10, 10, 11, 12, 13, 15, 10, 11, 12, 13, 14, 15, 16, 19]})

# TODO: implemente a resolução deste problema.
