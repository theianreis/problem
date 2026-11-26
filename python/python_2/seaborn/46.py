"""
Problema 46 — Contexto de apresentação
Área: seaborn | Nível: avançado

Enunciado:
Crie a mesma visualização em dois contextos temporários, paper e talk, usando sns.plotting_context.

Critérios de conclusão:
Os dados devem permanecer iguais; o contexto talk deve produzir textos maiores sem alterar permanentemente o contexto externo.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'dia': [1, 2, 3, 4], 'valor': [10, 15, 12, 20]})

# TODO: implemente a resolução deste problema.
