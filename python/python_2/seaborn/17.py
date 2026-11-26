"""
Problema 17 — Boxplot de salários
Área: seaborn | Nível: básico

Enunciado:
Compare salários por setor com boxplot e mantenha os valores extremos visíveis.

Critérios de conclusão:
Mostre duas caixas e um ponto extremo de 20000 no setor TI.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'setor': ['TI'] * 5 + ['RH'] * 5, 'salario': [4000, 4500, 5000, 5500, 20000, 3000, 3500, 4000, 4500, 5000]})

# TODO: implemente a resolução deste problema.
