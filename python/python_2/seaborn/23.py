"""
Problema 23 — Caixa com observações
Área: seaborn | Nível: intermediário

Enunciado:
Sobreponha boxplot e stripplot no mesmo Axes para combinar resumo e valores individuais.

Critérios de conclusão:
As categorias devem estar alinhadas e os pontos devem permanecer visíveis sobre as caixas.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'grupo': ['A'] * 6 + ['B'] * 6, 'valor': [1, 2, 3, 4, 5, 9, 3, 4, 5, 6, 7, 8]})

# TODO: implemente a resolução deste problema.
