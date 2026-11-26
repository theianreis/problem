"""
Problema 03 — Cor e marcador redundantes
Área: seaborn | Nível: básico

Enunciado:
Mostre custo e receita usando simultaneamente hue e style para o canal.

Critérios de conclusão:
Cada canal deve ser identificável tanto pela cor quanto pelo formato do marcador.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'custo': [10, 15, 20, 12, 18, 25], 'receita': [20, 28, 35, 25, 32, 45], 'canal': ['web'] * 3 + ['loja'] * 3})

# TODO: implemente a resolução deste problema.
