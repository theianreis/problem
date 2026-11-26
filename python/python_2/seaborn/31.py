"""
Problema 31 — Regressão linear
Área: seaborn | Nível: intermediário

Enunciado:
Use regplot para relacionar investimento e vendas e desenhe uma reta com intervalo de confiança de 95%.

Critérios de conclusão:
Mostre os pontos originais e use seed=42 para tornar o bootstrap reproduzível.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'investimento': [1, 2, 3, 4, 5, 6, 7, 8], 'venda': [3, 5, 6, 9, 10, 13, 14, 17]})

# TODO: implemente a resolução deste problema.
