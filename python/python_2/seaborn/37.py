"""
Problema 37 — Facetas de histogramas
Área: seaborn | Nível: avançado

Enunciado:
Use displot com col='grupo', bins compartilhados e kind='hist'.

Critérios de conclusão:
Mostre dois painéis com os mesmos limites de eixo x para permitir comparação direta.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'valor': [1, 2, 2, 3, 4, 5, 4, 5, 6, 6, 7, 8], 'grupo': ['A'] * 6 + ['B'] * 6})

# TODO: implemente a resolução deste problema.
