"""
Problema 45 — Paleta divergente centrada
Área: seaborn | Nível: avançado

Enunciado:
Mostre variações em relação à meta com heatmap e uma paleta divergente centrada em zero.

Critérios de conclusão:
Use limites simétricos -20 e 20; zero deve ter cor neutra e os sinais devem ser distinguíveis.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame([[-20, 0, 10], [5, -10, 20], [0, 15, -5]], index=['A', 'B', 'C'], columns=['Jan', 'Fev', 'Mar'])

# TODO: implemente a resolução deste problema.
