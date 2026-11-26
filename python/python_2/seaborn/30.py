"""
Problema 30 — Agrupamento hierárquico
Área: seaborn | Nível: intermediário

Enunciado:
Use clustermap para agrupar linhas e colunas de perfis numéricos sem valores ausentes.

Critérios de conclusão:
A e B devem ficar próximas, assim como C e D; use scipy quando necessário e explique a distância escolhida em comentário.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame([[1, 2, 1], [1.1, 2.1, 1.2], [8, 9, 8], [8.2, 9.1, 8.1]], index=['A', 'B', 'C', 'D'], columns=['x', 'y', 'z'])

# TODO: implemente a resolução deste problema.
