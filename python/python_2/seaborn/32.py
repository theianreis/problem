"""
Problema 32 — Regressão quadrática
Área: seaborn | Nível: intermediário

Enunciado:
Ajuste uma curva de segunda ordem com regplot para dados de crescimento não linear.

Critérios de conclusão:
Use order=2 e ci=None; a curva deve acompanhar o formato quadrático sem conectar os pontos em sequência.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'x': [-3, -2, -1, 0, 1, 2, 3], 'y': [9, 4, 1, 0, 1, 4, 9]})

# TODO: implemente a resolução deste problema.
