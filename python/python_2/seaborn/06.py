"""
Problema 06 — Linha com variabilidade
Área: seaborn | Nível: básico

Enunciado:
Plote a média de medições por dia e uma faixa de um desvio padrão com lineplot e errorbar='sd'.

Critérios de conclusão:
Cada dia tem três medições; a linha deve passar pelas médias 10, 15 e 20.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'dia': [1] * 3 + [2] * 3 + [3] * 3, 'valor': [8, 10, 12, 12, 15, 18, 16, 20, 24]})

# TODO: implemente a resolução deste problema.
