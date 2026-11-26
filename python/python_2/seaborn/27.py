"""
Problema 27 — Correlação anotada
Área: seaborn | Nível: intermediário

Enunciado:
Calcule a correlação entre as colunas numéricas e exiba um heatmap centrado em zero com limites -1 e 1.

Critérios de conclusão:
A diagonal deve ser 1; as correlações entre x e y e entre x e z devem ser 1 e -1.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'x': [1, 2, 3, 4, 5], 'y': [2, 4, 6, 8, 10], 'z': [10, 8, 6, 4, 2]})

# TODO: implemente a resolução deste problema.
