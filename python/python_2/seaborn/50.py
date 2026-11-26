"""
Problema 50 — Painel exploratório de vendas
Área: seaborn | Nível: avançado

Enunciado:
Crie um painel 2x2 usando funções de eixos do seaborn para faturamento por canal, distribuição de ticket, relação quantidade-ticket e correlação numérica.

Critérios de conclusão:
Calcule faturamento = quantidade * ticket antes de agregar; use soma nas barras e mantenha títulos e legenda consistentes.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'canal': ['web', 'loja'] * 5, 'quantidade': [1, 2, 3, 1, 2, 4, 3, 2, 1, 5], 'ticket': [20, 25, 30, 22, 28, 35, 32, 26, 24, 40]})

# TODO: implemente a resolução deste problema.
