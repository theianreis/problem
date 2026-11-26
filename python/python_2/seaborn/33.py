"""
Problema 33 — Análise de resíduos
Área: seaborn | Nível: intermediário

Enunciado:
Use residplot para mostrar resíduos de um ajuste linear e acrescente uma linha horizontal em zero.

Critérios de conclusão:
Os resíduos devem aparecer em torno da referência; identifique o padrão curvo que indica falta de ajuste linear.

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
