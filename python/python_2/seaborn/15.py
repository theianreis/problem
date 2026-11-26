"""
Problema 15 — Médias em barras
Área: seaborn | Nível: básico

Enunciado:
Use barplot para mostrar o salário médio por setor, sem barras de erro.

Critérios de conclusão:
As médias devem ser TI = 6000 e RH = 4500; deixe explícito no título que se trata de média.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'setor': ['TI', 'TI', 'RH', 'RH'], 'salario': [5000, 7000, 4000, 5000]})

# TODO: implemente a resolução deste problema.
