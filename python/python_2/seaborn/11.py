"""
Problema 11 — Distribuição acumulada
Área: seaborn | Nível: básico

Enunciado:
Use ecdfplot para comparar tempos de atendimento entre duas equipes.

Critérios de conclusão:
Cada curva deve ser não decrescente e atingir 1; identifique qual equipe concentra tempos menores.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'tempo': [2, 3, 4, 5, 6, 4, 5, 6, 7, 8], 'equipe': ['A'] * 5 + ['B'] * 5})

# TODO: implemente a resolução deste problema.
