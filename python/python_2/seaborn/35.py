"""
Problema 35 — Facetas de dispersão
Área: seaborn | Nível: avançado

Enunciado:
Use relplot com col='regiao' para separar a relação entre preço e demanda por região.

Critérios de conclusão:
Os dois painéis devem compartilhar escalas e exibir três observações cada.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'preco': [10, 20, 30] * 2, 'demanda': [100, 80, 50, 120, 90, 60], 'regiao': ['Sul'] * 3 + ['Norte'] * 3})

# TODO: implemente a resolução deste problema.
